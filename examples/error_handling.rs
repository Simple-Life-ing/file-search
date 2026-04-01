/// 错误处理示例
/// 
/// 演示如何处理搜索过程中可能出现的各种错误
/// 
/// 用法：
///   cargo run --example error_handling

use file_search::search::{search_in_file, SearchPattern};
use file_search::SearchError;
use std::fs;
use tempfile::TempDir;

fn main() {
    println!("=== File Search: 错误处理示例 ===\n");

    // 示例 1: 非存在的文件
    println!("📝 示例 1: 搜索不存在的文件");
    println!("尝试搜索: /nonexistent/file.txt\n");

    let pattern = SearchPattern::from_pattern("keyword", false).expect("Failed to create pattern");
    match search_in_file("/nonexistent/file.txt", &pattern) {
        Ok(()) => println!("✅ 搜索成功"),
        Err(SearchError::Io(e)) => {
            println!("❌ IO 错误: {}", e);
            println!("   错误类型: {:?}\n", e.kind());
        }
        Err(e) => println!("❌ 其他错误: {}\n", e),
    }

    // 示例 2: 成功搜索
    println!("📝 示例 2: 成功搜索");
    let dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = dir.path().join("success.txt");
    fs::write(&file_path, "This is a test file with content").expect("Failed to write file");
    let path_str = file_path.to_string_lossy().to_string();

    println!("在文件中搜索: {}\n", path_str);
    let pattern = SearchPattern::from_pattern("test", false).expect("Failed to create pattern");
    match search_in_file(&path_str, &pattern) {
        Ok(()) => println!("✅ 搜索成功，找到匹配项\n"),
        Err(e) => println!("❌ 错误: {}\n", e),
    }

    // 示例 3: 空文件
    println!("📝 示例 3: 搜索空文件");
    let empty_file = dir.path().join("empty.txt");
    fs::write(&empty_file, "").expect("Failed to create empty file");
    let empty_path = empty_file.to_string_lossy().to_string();

    println!("搜索空文件: {}\n", empty_path);
    let pattern = SearchPattern::from_pattern("anything", false).expect("Failed to create pattern");
    match search_in_file(&empty_path, &pattern) {
        Ok(()) => println!("✅ 搜索成功（无结果但无错误）\n"),
        Err(e) => println!("❌ 错误: {}\n", e),
    }

    // 示例 4: 文件读取错误演示
    println!("📝 示例 4: 文件读取失败场景（权限限制）");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        
        let restricted_file = dir.path().join("restricted.txt");
        fs::write(&restricted_file, "Restricted content").expect("Failed to write file");
        
        // 移除读权限
        let perms = fs::Permissions::from_mode(0o000);
        fs::set_permissions(&restricted_file, perms).expect("Failed to set permissions");

        let restricted_path = restricted_file.to_string_lossy().to_string();
        println!("尝试搜索无读权限的文件: {}\n", restricted_path);

        let pattern = SearchPattern::from_pattern("content", false).expect("Failed to create pattern");
        match search_in_file(&restricted_path, &pattern) {
            Ok(()) => println!("✅ 搜索成功"),
            Err(SearchError::Io(e)) => {
                println!("❌ IO 错误（权限被拒）: {}", e);
                println!("   错误类型: {:?}\n", e.kind());
            }
            Err(e) => println!("❌ 错误: {}\n", e),
        }

        // 恢复权限以便清理
        let perms = fs::Permissions::from_mode(0o644);
        fs::set_permissions(&restricted_file, perms).expect("Failed to restore permissions");
    }

    #[cfg(not(unix))]
    println!("（权限测试仅在 Unix 系统上运行）\n");

    // 示例 5: 多次搜索相同文件
    println!("📝 示例 5: 多次搜索演示");
    let test_file = dir.path().join("test.txt");
    fs::write(&test_file, "repeated content\nrepeated content\nrepeated content").expect("Failed");
    let test_path = test_file.to_string_lossy().to_string();

    let pattern = SearchPattern::from_pattern("repeated", false).expect("Failed to create pattern");
    for i in 1..=3 {
        println!("第 {} 次搜索:", i);
        match search_in_file(&test_path, &pattern) {
            Ok(()) => println!("  ✅ 搜索成功\n"),
            Err(e) => println!("  ❌ 错误: {}\n", e),
        }
    }

    println!("✨ 错误处理示例完成!");
}
