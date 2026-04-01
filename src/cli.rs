use clap::Parser;
use std::path::PathBuf;

/// 命令行参数定义
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// 配置文件路径（优先级低于命令行，若未指定则使用默认路径）
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// 要搜索的目录路径
    #[arg(value_name = "DIRECTORY")]
    pub directory: Option<PathBuf>,

    /// 要搜索的关键词
    #[arg(value_name = "KEYWORD")]
    pub keyword: Option<String>,

    /// 使用的线程数（默认为 CPU 核心数）
    #[arg(short, long)]
    pub threads: Option<usize>,

    /// 显示详细信息
    #[arg(short, long)]
    pub verbose: bool,

    /// 日志级别 (error, warn, info, debug, trace)
    #[arg(long)]
    pub log_level: Option<String>,

    /// 包含的扩展名，多个用逗号分隔（如 txt,md）
    #[arg(long, value_delimiter = ',')]
    pub include_ext: Option<Vec<String>>,

    /// 排除的扩展名，多个用逗号分隔（如 log,tmp）
    #[arg(long, value_delimiter = ',')]
    pub exclude_ext: Option<Vec<String>>,
}

impl Args {
    pub fn parse_args() -> Self {
        Args::parse()
    }
}
