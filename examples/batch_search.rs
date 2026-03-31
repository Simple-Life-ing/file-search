/// 批量搜索示例
/// 
/// 演示在多个文件中批量搜索相同的关键词
/// 
/// 用法：
///   cargo run --example batch_search

use file_search::search::search_in_file;
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tempfile::TempDir;

fn main() {
    println!("=== File Search: 批量搜索示例 ===\n");

    // 创建临时目录和多个示例文件
    let dir = TempDir::new().expect("Failed to create temp dir");

    let files = vec![
        ("README.md", "# Project Documentation\nThis project uses Rust\nRust is safe and fast"),
        ("src/main.rs", "fn main() {\n    println!(\"Hello, Rust!\");\n}"),
        ("Cargo.toml", "[package]\nname = \"my-project\"\nRust edition 2024"),
        ("notes.txt", "Remember to learn Rust\nRust has great documentation\nCommunity is helpful"),
    ];

    let mut file_paths = Vec::new();

    // 创建文件
    for (filename, content) in &files {
        let file_path = dir.path().join(filename);
        
        // 创建必要的子目录
        if let Some(parent) = file_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).expect("Failed to create directory");
            }
        }
        
        fs::write(&file_path, content).expect("Failed to write file");
        file_paths.push((file_path.to_string_lossy().to_string(), filename.to_string()));
    }

    println!("📁 创建了 {} 个示例文件:\n", file_paths.len());
    for (_, name) in &file_paths {
        println!("  - {}", name);
    }

    // 定义要搜索的关键词
    let keyword = "Rust";
    println!("\n🔍 在所有文件中搜索关键词: \"{}\"\n", keyword);

    // 计数搜索结果
    let match_count = Arc::new(AtomicUsize::new(0));
    let mut found_files = Vec::new();

    for (path, name) in &file_paths {
        println!("📄 搜索文件: {}", name);
        match search_in_file(path, keyword) {
            Ok(()) => {
                found_files.push(name.clone());
                match_count.fetch_add(1, Ordering::Relaxed);
                println!("   ✅ 找到匹配项\n");
            }
            Err(e) => println!("   ❌ 错误: {}\n", e),
        }
    }

    // 总结结果
    println!("\n📊 搜索总结:");
    println!("  - 总文件数: {}", file_paths.len());
    println!("  - 匹配文件数: {}", match_count.load(Ordering::Relaxed));
    println!("  - 匹配的文件:\n");
    for name in found_files {
        println!("    - {}", name);
    }

    println!("\n✨ 示例完成!");
}
