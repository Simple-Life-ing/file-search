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

pub mod cli;
pub mod config;
pub mod error;
pub mod executor;
pub mod search;

pub use cli::Args;
pub use config::{load_configuration, default_config_path, load_file_config, RuntimeConfig};
pub use error::{SearchError, Result};
pub use executor::{run as execute_search};

