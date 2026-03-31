use clap::Parser;
use rayon::prelude::*;
use std::path::PathBuf;
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
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(&args) {
        eprintln!("❌ 错误: {}", e);
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
        eprintln!("📁 搜索目录: {}", args.directory.display());
        eprintln!("🔍 搜索关键词: {}", args.keyword);
    }

    let entries: Vec<_> = WalkDir::new(dir_str)
        .into_iter()
        .filter_map(|e| {
            e.map_err(|err| {
                if args.verbose {
                    eprintln!("⚠️  目录遍历错误: {}", err);
                }
                err
            })
            .ok()
        })
        .collect();

    // 如果没有找到任何文件，直接返回
    if entries.is_empty() {
        if args.verbose {
            eprintln!("⚠️  未找到任何文件");
        }
        return Ok(());
    }

    let num_threads = args.threads.unwrap_or_else(|| {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    });

    if args.verbose {
        eprintln!("⚙️  使用线程数: {}", num_threads);
        eprintln!("📊 扫描文件数: {}", entries.len());
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()?;

    pool.install(|| {
        entries.par_iter().for_each(|entry| {
            if entry.file_type().is_file() {
                let path = entry.path().display().to_string();
                // 忽略单个文件的搜索错误，继续处理下一个文件
                if let Err(e) = search::search_in_file(&path, &args.keyword) {
                    if args.verbose {
                        eprintln!("⚠️  搜索文件 {} 失败: {}", path, e);
                    }
                }
            }
        });
    });

    if args.verbose {
        eprintln!("✅ 搜索完成");
    }

    Ok(())
}
