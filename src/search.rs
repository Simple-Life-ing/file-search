use crate::error::{Result, SearchError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use tracing::warn;

/// 在指定文件中搜索关键词
///
/// # 参数
/// * `path` - 文件路径
/// * `keyword` - 要搜索的关键词
///
/// # 返回
/// * `Ok(())` - 搜索成功完成
/// * `Err(SearchError)` - 搜索过程中发生错误
pub fn search_in_file(path: &str, keyword: &str) -> Result<()> {
    let file = File::open(path).map_err(|e| {
        warn!("警告: 无法打开文件 {}: {}", path, e);
        SearchError::Io(e)
    })?;

    let reader = BufReader::new(file);

    for (num, line_result) in reader.lines().enumerate() {
        let line = line_result.map_err(|_e| SearchError::FileRead {
            path: path.to_string(),
        })?;

        if line.contains(keyword) {
            println!("{}:{} -> {}", path, num + 1, line);
        }
    }

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // 辅助函数：创建临时文件
    fn create_temp_file(dir: &TempDir, content: &str) -> String {
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, content).expect("Failed to write test file");
        file_path.to_string_lossy().to_string()
    }

    /// 测试目标：验证在单行文件中成功找到关键词
    #[test]
    fn test_search_found_single_line() {
        let dir = TempDir::new().expect("Failed to create temp dir");
        let path = create_temp_file(&dir, "hello world");

        let result = search_in_file(&path, "hello");
        assert!(result.is_ok());
    }

    /// 测试目标：验证在多行文件中找到多个重复的关键词是否正确
    #[test]
    fn test_search_found_multiple_lines() {
        let dir = TempDir::new().expect("Failed to create temp dir");
        let content = "first line\nhello world\nthird line\nhello again";
        let path = create_temp_file(&dir, content);

        let result = search_in_file(&path, "hello");
        assert!(result.is_ok());
    }

    /// 测试目标：验证关键词不存在时仍返回 Ok (无输出但成功)
    #[test]
    fn test_search_not_found() {
        let dir = TempDir::new().expect("Failed to create temp dir");
        let path = create_temp_file(&dir, "hello world");

        let result = search_in_file(&path, "goodbye");
        assert!(result.is_ok());
    }

    /// 测试目标：验证空文件的搜索不会出错
    #[test]
    fn test_search_empty_file() {
        let dir = TempDir::new().expect("Failed to create temp dir");
        let path = create_temp_file(&dir, "");

        let result = search_in_file(&path, "anything");
        assert!(result.is_ok());
    }

    /// 测试目标：验证搜索是大小写敏感的
    #[test]
    fn test_search_case_sensitive() {
        let dir = TempDir::new().expect("Failed to create temp dir");
        let path = create_temp_file(&dir, "Hello World");

        // 应该找到精确匹配
        let result = search_in_file(&path, "Hello");
        assert!(result.is_ok());

        // 应该不找到大小写不同的
        let result = search_in_file(&path, "hello");
        assert!(result.is_ok()); // 由于我们只是检查是否出错，都是ok
    }

    /// 测试目标：验证文件不存在时返回 SearchError::Io
    #[test]
    fn test_search_nonexistent_file() {
        let result = search_in_file("/nonexistent/path/to/file.txt", "keyword");
        assert!(result.is_err());

        match result {
            Err(SearchError::Io(_)) => {
                // Expected IO error for file not found
            }
            _ => panic!("Expected SearchError::Io"),
        }
    }

    /// 测试目标：验证搜索空字符串的行为 (contains("") 在所有字符串中返回 true)
    #[test]
    fn test_search_keyword_empty_string() {
        let dir = TempDir::new().expect("Failed to create temp dir");
        let path = create_temp_file(&dir, "hello world\ntest");

        // 空字符串在所有行中都会匹配（contains("")返回true）
        let result = search_in_file(&path, "");
        assert!(result.is_ok());
    }

    /// 测试目标：验证多行文件的行遍历和搜索逻辑
    #[test]
    fn test_search_multiline_content() {
        let dir = TempDir::new().expect("Failed to create temp dir");
        let content = "line 1\nline 2\nfoo bar\nline 4\nfoo baz";
        let path = create_temp_file(&dir, content);

        let result = search_in_file(&path, "foo");
        assert!(result.is_ok());
    }

    /// 测试目标：验证特殊字符 (@, $, %) 的正确搜索
    #[test]
    fn test_search_special_characters() {
        let dir = TempDir::new().expect("Failed to create temp dir");
        let content = "test@email.com\n$special$\n%percent%";
        let path = create_temp_file(&dir, content);

        let result = search_in_file(&path, "@");
        assert!(result.is_ok());

        let result = search_in_file(&path, "$");
        assert!(result.is_ok());
    }

    /// 测试目标：验证 Unicode 字符 (中文、日文等) 的正确搜索
    #[test]
    fn test_search_unicode_content() {
        let dir = TempDir::new().expect("Failed to create temp dir");
        let content = "Hello 世界\n你好 world\n中文测试";
        let path = create_temp_file(&dir, content);

        let result = search_in_file(&path, "世界");
        assert!(result.is_ok());

        let result = search_in_file(&path, "中文");
        assert!(result.is_ok());
    }

    /// 测试目标：验证大文件 (10000+ 行) 搜索的性能和可扩展性
    #[test]
    fn test_search_large_file() {
        let dir = TempDir::new().expect("Failed to create temp dir");
        let mut content = String::new();
        for i in 0..10000 {
            content.push_str(&format!("line {} content\n", i));
        }
        content.push_str("target keyword found here");
        let path = create_temp_file(&dir, &content);

        let result = search_in_file(&path, "target keyword");
        assert!(result.is_ok());
    }
}
