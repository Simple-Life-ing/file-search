/// 基础搜索示例
///
/// 演示 file-search 库的最简单用法：在单个文件中搜索关键词
///
/// 用法：
///   cargo run --example basic_search
use file_search::search::{search_in_file, SearchPattern};
use std::fs;
use tempfile::TempDir;

fn main() {
    println!("=== File Search: 基础搜索示例 ===\n");

    // 创建临时目录和示例文件
    let dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = dir.path().join("example.txt");

    let content = r#"Rust 编程语言
Rust 是一门系统编程语言
Rust 强调安全、速度和并发
Python 是一门高级编程语言
Java 也很流行
Rust 拥有强大的包管理器 Cargo
"#;

    fs::write(&file_path, content).expect("Failed to write file");
    let file_path_str = file_path.to_string_lossy().to_string();

    println!("📄 创建示例文件: {}", file_path_str);
    println!("📝 文件内容:\n{}\n", content);

    // 搜索关键词
    let keywords = vec!["Rust", "Python", "Go"];

    for keyword in keywords {
        println!("🔍 搜索关键词: \"{}\"", keyword);
        match SearchPattern::from_pattern(keyword, false) {
            Ok(pattern) => {
                match search_in_file(&file_path_str, &pattern) {
                    Ok(()) => println!("✅ 搜索完成\n"),
                    Err(e) => println!("❌ 错误: {}\n", e),
                }
            }
            Err(e) => println!("❌ 模式错误: {}\n", e),
        }
    }

    println!("✨ 示例完成!");
}
