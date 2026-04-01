//! 高层 API
//!
//! 使用示例：
//!
//! ```no_run
//! use file_search::{Args, execute_search};
//!
//! let args = Args::parse_args();
//! let runtime_config = file_search::config::load_configuration(&args).unwrap();
//! execute_search(&runtime_config).unwrap();
//! ```
//!
//! 字面量搜索示例：
//!
//! ```no_run
//! // cargo run -- /path/to/search "error"
//! ```
//!
//! 正则表达式搜索示例：
//!
//! ```no_run
//! // cargo run -- /path/to/search "error|warn" --regex
//! // cargo run -- /path/to/search r"\d{3}-\d{4}" --regex
//! ```
//!
//! 配置文件示例（~/.config/file-search/config.toml）:
//!
//! ```toml
//! # 字面量搜索
//! directory = "/path/to/search"
//! keyword = "todo"
//! regex = false
//!
//! # 正则表达式搜索
//! # directory = "/path/to/search"
//! # keyword = r"\[TODO\].*"
//! # regex = true
//! ```

pub mod cli;
pub mod config;
pub mod error;
pub mod executor;
pub mod search;

pub use cli::Args;
pub use config::{load_configuration, default_config_path, load_file_config, RuntimeConfig};
pub use error::{SearchError, Result};
pub use executor::{run as execute_search};
pub use search::SearchPattern;

