use std::env;
use std::thread;
use walkdir::WalkDir;

mod error;
mod search;

use error::SearchError;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "Usage: {} <directory> <keyword>",
            args.first().map_or("file-search", |s| s)
        );
        std::process::exit(1);
    }

    let dir = match args.get(1) {
        Some(d) => d,
        None => {
            eprintln!("错误: 目录参数缺失");
            std::process::exit(1);
        }
    };

    let keyword = match args.get(2) {
        Some(k) => k,
        None => {
            eprintln!("错误: 关键词参数缺失");
            std::process::exit(1);
        }
    };

    if let Err(e) = run(dir, keyword) {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}

fn run(dir: &str, keyword: &str) -> Result<(), SearchError> {
    let entries: Vec<_> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| {
            e.map_err(|err| {
                eprintln!("警告: 目录遍历错误: {}", err);
                err
            })
            .ok()
        })
        .collect();

    // 如果没有找到任何文件，直接返回
    if entries.is_empty() {
        eprintln!("警告: 未找到任何文件");
        return Ok(());
    }

    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4);
    let chunk_size = entries.len().div_ceil(num_threads);

    thread::scope(|s| {
        for chunk in entries.chunks(chunk_size) {
            s.spawn(move || {
                for entry in chunk {
                    if entry.file_type().is_file() {
                        let path = entry.path().display().to_string();
                        // 忽略单个文件的搜索错误，继续处理下一个文件
                        if let Err(e) = search::search_in_file(&path, keyword) {
                            eprintln!("警告: 搜索文件 {} 失败: {}", path, e);
                        }
                    }
                }
            });
        }
    });

    Ok(())
}
