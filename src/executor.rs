use crate::config::RuntimeConfig;
use crate::error::SearchError;
use crate::search;
use rayon::prelude::*;
use tracing::{info, warn};
use walkdir::WalkDir;

/// 执行搜索主流程
///
/// Usage:
///
/// ```
/// let runtime_config = file_search::config::load_configuration(&args)?;
/// file_search::executor::run(&runtime_config)?;
/// ```
pub fn run(config: &RuntimeConfig) -> Result<(), SearchError> {
    // 验证目录是否存在
    if !config.directory.exists() {
        return Err(SearchError::InvalidPath(format!("目录不存在: {:?}", config.directory)));
    }

    if !config.directory.is_dir() {
        return Err(SearchError::InvalidPath(format!("路径不是目录: {:?}", config.directory)));
    }

    let dir_str = config
        .directory
        .to_str()
        .ok_or_else(|| SearchError::PathEncoding("无法转换路径编码".to_string()))?;

    if config.verbose {
        info!("📁 搜索目录: {}", config.directory.display());
        info!("🔍 搜索关键词: {}", config.keyword);

        if !config.include_ext.is_empty() {
            info!("📥 包含扩展: {}", config.include_ext.join(", "));
        }
        if !config.exclude_ext.is_empty() {
            info!("📤 排除扩展: {}", config.exclude_ext.join(", "));
        }
    }

    let entries: Vec<_> = WalkDir::new(dir_str)
        .into_iter()
        .filter_map(|e| {
            e.map_err(|err| {
                if config.verbose {
                    warn!("⚠️  目录遍历错误: {}", err);
                }
                err
            })
            .ok()
        })
        .collect();

    if entries.is_empty() {
        if config.verbose {
            warn!("⚠️  未找到任何文件");
        }
        return Ok(());
    }

    let num_threads = config.threads;

    if config.verbose {
        info!("⚙️  使用线程数: {}", num_threads);
        info!("📊 扫描文件数: {}", entries.len());
    }

    let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build()?;

    let include_ext = config.include_ext.clone();
    let exclude_ext = config.exclude_ext.clone();

    pool.install(|| {
        entries.par_iter().for_each(|entry| {
            if entry.file_type().is_file() {
                let path_obj = entry.path();
                if !file_extension_allowed(path_obj, &include_ext, &exclude_ext) {
                    if config.verbose {
                        info!("⏭️  跳过文件(扩展不符): {}", path_obj.display());
                    }
                    return;
                }

                let path = path_obj.display().to_string();
                if let Err(e) = search::search_in_file(&path, &config.keyword) {
                    if config.verbose {
                        warn!("⚠️  搜索文件 {} 失败: {}", path, e);
                    }
                }
            }
        });
    });

    if config.verbose {
        info!("✅ 搜索完成");
    }

    Ok(())
}

/// 依据 include/exclude 规则判断扩展名可否搜索
///
/// Usage:
///
/// ```
/// assert!(file_search::executor::file_extension_allowed(Path::new("foo.rs"), &[], &[]));
/// ```
pub fn file_extension_allowed(
    path: &std::path::Path,
    include_ext: &[String],
    exclude_ext: &[String],
) -> bool {
    let ext_opt = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase());

    if let Some(ext) = &ext_opt {
        if !exclude_ext.is_empty() && exclude_ext.contains(ext) {
            return false;
        }

        if !include_ext.is_empty() && !include_ext.contains(ext) {
            return false;
        }

        true
    } else {
        include_ext.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// 测试目标：无 include/exclude 条件时，所有文件类型均被允许
    #[test]
    fn test_file_extension_allowed_no_rules() {
        let include: Vec<String> = vec![];
        let exclude: Vec<String> = vec![];

        assert!(file_extension_allowed(Path::new("file.txt"), &include, &exclude));
        assert!(file_extension_allowed(Path::new("file.md"), &include, &exclude));
        assert!(file_extension_allowed(Path::new("file"), &include, &exclude));
    }

    /// 测试目标：仅允许包含列表中的扩展名，其它扩展名和无扩展名应被拒绝
    #[test]
    fn test_file_extension_allowed_include() {
        let include = vec!["txt".to_string(), "md".to_string()];
        let exclude: Vec<String> = vec![];

        assert!(file_extension_allowed(Path::new("file.txt"), &include, &exclude));
        assert!(file_extension_allowed(Path::new("file.md"), &include, &exclude));
        assert!(!file_extension_allowed(Path::new("file.log"), &include, &exclude));
        assert!(!file_extension_allowed(Path::new("file"), &include, &exclude));
    }

    /// 测试目标：排除列表中扩展名应拒绝，其他扩展名和无扩展名应允许
    #[test]
    fn test_file_extension_allowed_exclude() {
        let include: Vec<String> = vec![];
        let exclude = vec!["log".to_string(), "tmp".to_string()];

        assert!(!file_extension_allowed(Path::new("file.log"), &include, &exclude));
        assert!(!file_extension_allowed(Path::new("file.tmp"), &include, &exclude));
        assert!(file_extension_allowed(Path::new("file.txt"), &include, &exclude));
        assert!(file_extension_allowed(Path::new("file"), &include, &exclude));
    }

    /// 测试目标：当同时存在 include 和 exclude 时，exclude 优先级高于 include
    #[test]
    fn test_file_extension_allowed_both_rules() {
        let include = vec!["txt".to_string(), "md".to_string()];
        let exclude = vec!["txt".to_string()];

        assert!(!file_extension_allowed(Path::new("file.txt"), &include, &exclude));
        assert!(file_extension_allowed(Path::new("file.md"), &include, &exclude));
        assert!(!file_extension_allowed(Path::new("file.log"), &include, &exclude));
    }
}
