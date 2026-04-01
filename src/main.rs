use clap::Parser;
use rayon::prelude::*;
use std::path::PathBuf;
use tracing::{error, info, warn};
use tracing_subscriber;
use walkdir::WalkDir;

mod error;
mod search;

use error::SearchError;

/// 文件搜索工具 - 在指定目录中递归搜索关键词
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 要搜索的目录路径
    #[arg(value_name = "DIRECTORY")]
    directory: PathBuf,

    /// 要搜索的关键词
    #[arg(value_name = "KEYWORD")]
    keyword: String,

    /// 使用的线程数（默认为 CPU 核心数）
    #[arg(short, long)]
    threads: Option<usize>,

    /// 显示详细信息
    #[arg(short, long)]
    verbose: bool,

    /// 日志级别 (error, warn, info, debug, trace)
    #[arg(long, default_value = "info")]
    log_level: String,

    /// 包含的扩展名，多个用逗号分隔（如 txt,md）
    #[arg(long, value_delimiter = ',')]
    include_ext: Option<Vec<String>>,

    /// 排除的扩展名，多个用逗号分隔（如 log,tmp）
    #[arg(long, value_delimiter = ',')]
    exclude_ext: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();

    // 初始化日志系统
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&args.log_level)),
        )
        .init();

    if let Err(e) = run(&args) {
        error!("❌ 错误: {}", e);
        std::process::exit(1);
    }
}

fn run(args: &Args) -> Result<(), SearchError> {
    // 验证目录是否存在
    if !args.directory.exists() {
        return Err(SearchError::InvalidPath(format!(
            "目录不存在: {:?}",
            args.directory
        )));
    }

    if !args.directory.is_dir() {
        return Err(SearchError::InvalidPath(format!(
            "路径不是目录: {:?}",
            args.directory
        )));
    }

    let dir_str = args
        .directory
        .to_str()
        .ok_or_else(|| SearchError::PathEncoding("无法转换路径编码".to_string()))?;

    if args.verbose {
        info!("📁 搜索目录: {}", args.directory.display());
        info!("🔍 搜索关键词: {}", args.keyword);

        if let Some(include_ext) = &args.include_ext {
            info!("📥 包含扩展: {}", include_ext.join(", "));
        }
        if let Some(exclude_ext) = &args.exclude_ext {
            info!("📤 排除扩展: {}", exclude_ext.join(", "));
        }
    }

    let entries: Vec<_> = WalkDir::new(dir_str)
        .into_iter()
        .filter_map(|e| {
            e.map_err(|err| {
                if args.verbose {
                    warn!("⚠️  目录遍历错误: {}", err);
                }
                err
            })
            .ok()
        })
        .collect();

    // 如果没有找到任何文件，直接返回
    if entries.is_empty() {
        if args.verbose {
            warn!("⚠️  未找到任何文件");
        }
        return Ok(());
    }

    let num_threads = args.threads.unwrap_or_else(|| {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    });

    if args.verbose {
        info!("⚙️  使用线程数: {}", num_threads);
        info!("📊 扫描文件数: {}", entries.len());
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()?;

    let include_ext = args
        .include_ext
        .clone()
        .unwrap_or_default()
        .into_iter()
        .map(|ext| ext.to_lowercase())
        .collect::<Vec<_>>();

    let exclude_ext = args
        .exclude_ext
        .clone()
        .unwrap_or_default()
        .into_iter()
        .map(|ext| ext.to_lowercase())
        .collect::<Vec<_>>();

    pool.install(|| {
        entries.par_iter().for_each(|entry| {
            if entry.file_type().is_file() {
                let path_obj = entry.path();
                if !file_extension_allowed(path_obj, &include_ext, &exclude_ext) {
                    if args.verbose {
                        info!("⏭️  跳过文件(扩展不符): {}", path_obj.display());
                    }
                    return;
                }

                let path = path_obj.display().to_string();
                // 忽略单个文件的搜索错误，继续处理下一个文件
                if let Err(e) = search::search_in_file(&path, &args.keyword) {
                    if args.verbose {
                        warn!("⚠️  搜索文件 {} 失败: {}", path, e);
                    }
                }
            }
        });
    });

    if args.verbose {
        info!("✅ 搜索完成");
    }

    Ok(())
}

fn file_extension_allowed(
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
        // 无扩展名时，只有 include_ext 为空时才允许
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

        assert!(file_extension_allowed(
            Path::new("file.txt"),
            &include,
            &exclude
        ));
        assert!(file_extension_allowed(
            Path::new("file.md"),
            &include,
            &exclude
        ));
        assert!(file_extension_allowed(
            Path::new("file"),
            &include,
            &exclude
        ));
    }

    /// 测试目标：仅允许包含列表中的扩展名，其它扩展名和无扩展名应被拒绝
    #[test]
    fn test_file_extension_allowed_include() {
        let include = vec!["txt".to_string(), "md".to_string()];
        let exclude: Vec<String> = vec![];

        assert!(file_extension_allowed(
            Path::new("file.txt"),
            &include,
            &exclude
        ));
        assert!(file_extension_allowed(
            Path::new("file.md"),
            &include,
            &exclude
        ));
        assert!(!file_extension_allowed(
            Path::new("file.log"),
            &include,
            &exclude
        ));
        assert!(!file_extension_allowed(
            Path::new("file"),
            &include,
            &exclude
        ));
    }

    /// 测试目标：排除列表中扩展名应拒绝，其他扩展名和无扩展名应允许
    #[test]
    fn test_file_extension_allowed_exclude() {
        let include: Vec<String> = vec![];
        let exclude = vec!["log".to_string(), "tmp".to_string()];

        assert!(!file_extension_allowed(
            Path::new("file.log"),
            &include,
            &exclude
        ));
        assert!(!file_extension_allowed(
            Path::new("file.tmp"),
            &include,
            &exclude
        ));
        assert!(file_extension_allowed(
            Path::new("file.txt"),
            &include,
            &exclude
        ));
        assert!(file_extension_allowed(
            Path::new("file"),
            &include,
            &exclude
        ));
    }

    /// 测试目标：当同时存在 include 和 exclude 时，exclude 优先级高于 include
    #[test]
    fn test_file_extension_allowed_both_rules() {
        let include = vec!["txt".to_string(), "md".to_string()];
        let exclude = vec!["txt".to_string()];

        assert!(!file_extension_allowed(
            Path::new("file.txt"),
            &include,
            &exclude
        ));
        assert!(file_extension_allowed(
            Path::new("file.md"),
            &include,
            &exclude
        ));
        assert!(!file_extension_allowed(
            Path::new("file.log"),
            &include,
            &exclude
        ));
    }
}
