use clap::Parser;
use std::path::PathBuf;
use tracing::error;
use tracing_subscriber;

mod config;
mod error;
mod executor;
mod search;

use error::SearchError;

/// 文件搜索工具 - 在指定目录中递归搜索关键词
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 配置文件路径（优先级低于命令行，若未指定则使用默认路径）
    #[arg(long)]
    config: Option<PathBuf>,

    /// 要搜索的目录路径
    #[arg(value_name = "DIRECTORY")]
    directory: Option<PathBuf>,

    /// 要搜索的关键词
    #[arg(value_name = "KEYWORD")]
    keyword: Option<String>,

    /// 使用的线程数（默认为 CPU 核心数）
    #[arg(short, long)]
    threads: Option<usize>,

    /// 显示详细信息
    #[arg(short, long)]
    verbose: bool,

    /// 日志级别 (error, warn, info, debug, trace)
    #[arg(long)]
    log_level: Option<String>,

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
    let runtime_log_level = args
        .log_level
        .clone()
        .or_else(|| {
            config::default_config_path()
                .and_then(|path| config::load_file_config(&path).ok())
                .and_then(|cfg| cfg.log_level)
        })
        .unwrap_or_else(|| "info".to_string());

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&runtime_log_level)),
        )
        .init();

    if let Err(e) = run_from_args(&args) {
        error!("❌ 错误: {}", e);
        std::process::exit(1);
    }
}

fn run_from_args(args: &Args) -> Result<(), SearchError> {
    let config = config::load_configuration(args)?;
    executor::run(&config)
}
