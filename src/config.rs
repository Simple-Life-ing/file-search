use std::path::PathBuf;

use crate::error::SearchError;

#[derive(Debug, serde::Deserialize, Default)]
pub struct FileConfig {
    pub directory: Option<PathBuf>,
    pub keyword: Option<String>,
    pub threads: Option<usize>,
    pub verbose: Option<bool>,
    pub log_level: Option<String>,
    pub include_ext: Option<Vec<String>>,
    pub exclude_ext: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct RuntimeConfig {
    pub directory: PathBuf,
    pub keyword: String,
    pub threads: usize,
    pub verbose: bool,
    pub include_ext: Vec<String>,
    pub exclude_ext: Vec<String>,
}

/// 获取默认配置文件路径，来自 HOME 环境变量
///
/// Usage:
///
/// ```
/// let path = file_search::config::default_config_path();
/// ```
pub fn default_config_path() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|home| {
        let mut p = PathBuf::from(home);
        p.push(".config/file-search/config.toml");
        p
    })
}

/// 从 TOML 文件载入配置
///
/// Usage:
///
/// ```no_run
/// use std::path::PathBuf;
/// let path = PathBuf::from("/path/to/config.toml");
/// let cfg = file_search::config::load_file_config(&path).unwrap();
/// ```
pub fn load_file_config(path: &PathBuf) -> Result<FileConfig, SearchError> {
    let content = std::fs::read_to_string(path).map_err(SearchError::Io)?;
    toml::from_str(&content).map_err(|e| SearchError::ConfigParse(e.to_string()))
}

/// 从命令行 + 配置文件生成运行时配置
///
/// Usage:
///
/// ```no_run
/// use file_search::{Args, execute_search};
/// 
/// let args = Args::parse_args();
/// let runtime = file_search::config::load_configuration(&args).unwrap();
/// execute_search(&runtime).unwrap();
/// ```
///
/// 只使用配置文件：
///
/// ```no_run
/// // 直接通过命令行：
/// // cargo run -- --config ~/.config/file-search/config.toml
/// ```
///
/// 配置示例(`~/.config/file-search/config.toml`):
///
/// ```toml
/// directory = "/path/to/search"
/// keyword = "todo"
/// threads = 4
/// verbose = true
/// include_ext = ["rs", "md"]
/// exclude_ext = ["log"]
/// ```
pub fn load_configuration(args: &crate::cli::Args) -> Result<RuntimeConfig, SearchError> {
    let mut file_config = FileConfig::default();

    let config_path = if let Some(cfg) = args.config.as_ref() {
        Some(cfg.clone())
    } else if let Some(default_path) = default_config_path() {
        if default_path.exists() {
            Some(default_path)
        } else {
            None
        }
    } else {
        None
    };

    if let Some(path) = config_path {
        file_config = load_file_config(&path)?;
    }

    let directory = args
        .directory
        .clone()
        .or(file_config.directory)
        .ok_or_else(|| SearchError::MissingParameter("directory".into()))?;

    let keyword = args
        .keyword
        .clone()
        .or(file_config.keyword)
        .ok_or_else(|| SearchError::MissingParameter("keyword".into()))?;

    let threads = args.threads.or(file_config.threads).unwrap_or_else(|| {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    });

    let verbose = args.verbose || file_config.verbose.unwrap_or(false);

    let include_ext = args
        .include_ext
        .clone()
        .or(file_config.include_ext)
        .unwrap_or_default()
        .into_iter()
        .map(|ext| ext.to_lowercase())
        .collect();

    let exclude_ext = args
        .exclude_ext
        .clone()
        .or(file_config.exclude_ext)
        .unwrap_or_default()
        .into_iter()
        .map(|ext| ext.to_lowercase())
        .collect();

    Ok(RuntimeConfig {
        directory,
        keyword,
        threads,
        verbose,
        include_ext,
        exclude_ext,
    })
}

//
// Usage Examples:
// With config only: cargo run -- --config ~/.config/file-search/config.toml
// CLI override: cargo run -- --config ~/.config/file-search/config.toml . search
// Custom config: cargo run -- --config /path/to/custom.toml . keyword
//
