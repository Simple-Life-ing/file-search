use crate::error::{Result, SearchError};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use tracing::warn;

/// 搜索模式枚举，支持字面量和正则表达式两种模式
#[derive(Debug, Clone)]
pub enum SearchPattern {
    /// 字面量搜索（精确字符串匹配）
    Literal(String),
    /// 正则表达式搜索
    Regex(Arc<Regex>),
}

impl SearchPattern {
    /// 从字符串构建搜索模式
    ///
    /// # 参数
    /// * `pattern` - 搜索模式字符串
    /// * `use_regex` - 是否使用正则表达式
    ///
    /// # 返回
    /// * `Ok(SearchPattern)` - 成功构建的搜索模式
    /// * `Err(SearchError)` - 如果正则表达式编译失败
    ///
    /// # 示例
    /// ```
    /// use file_search::search::SearchPattern;
    ///
    /// // 字面量模式
    /// let literal = SearchPattern::from_pattern("test", false).unwrap();
    /// assert!(matches!(literal, SearchPattern::Literal(_)));
    ///
    /// // 正则模式
    /// let regex = SearchPattern::from_pattern(r"\d+", true).unwrap();
    /// assert!(matches!(regex, SearchPattern::Regex(_)));
    /// ```
    pub fn from_pattern(pattern: &str, use_regex: bool) -> Result<Self> {
        if use_regex {
            let regex =
                Regex::new(pattern).map_err(|e| SearchError::RegexCompile(e.to_string()))?;
            Ok(SearchPattern::Regex(Arc::new(regex)))
        } else {
            Ok(SearchPattern::Literal(pattern.to_string()))
        }
    }

    /// 判断文本是否匹配搜索模式
    ///
    /// # 参数
    /// * `text` - 要检查的文本
    ///
    /// # 返回
    /// 如果文本匹配搜索模式返回 true，否则返回 false
    pub fn is_match(&self, text: &str) -> bool {
        match self {
            SearchPattern::Literal(keyword) => text.contains(keyword),
            SearchPattern::Regex(regex) => regex.is_match(text),
        }
    }

    /// 获取搜索模式的显示名称
    pub fn pattern_type(&self) -> &'static str {
        match self {
            SearchPattern::Literal(_) => "文字搜索",
            SearchPattern::Regex(_) => "正则搜索",
        }
    }
}

/// 在指定文件中搜索模式
///
/// # 参数
/// * `path` - 文件路径
/// * `pattern` - 搜索模式
///
/// # 返回
/// * `Ok(())` - 搜索成功完成
/// * `Err(SearchError)` - 搜索过程中发生错误
///
/// # 示例
/// ```no_run
/// use file_search::search::{SearchPattern, search_in_file};
///
/// let pattern = SearchPattern::from_pattern("error", false).unwrap();
/// search_in_file("log.txt", &pattern).unwrap();
/// ```
pub fn search_in_file(path: &str, pattern: &SearchPattern) -> Result<()> {
    let file = File::open(path).map_err(|e| {
        warn!("警告: 无法打开文件 {}: {}", path, e);
        SearchError::Io(e)
    })?;

    let reader = BufReader::new(file);

    for (num, line_result) in reader.lines().enumerate() {
        let line = line_result.map_err(|_e| SearchError::FileRead {
            path: path.to_string(),
        })?;

        if pattern.is_match(&line) {
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

    /// 测试目标：验证从模式字符串构建字面量模式
    #[test]
    fn test_literal_pattern_creation() {
        let pattern = SearchPattern::from_pattern("test", false).unwrap();
        assert!(matches!(pattern, SearchPattern::Literal(_)));
        assert_eq!(pattern.pattern_type(), "文字搜索");
    }

    /// 测试目标：验证从模式字符串构建正则模式
    #[test]
    fn test_regex_pattern_creation() {
        let pattern = SearchPattern::from_pattern(r"\d+", true).unwrap();
        assert!(matches!(pattern, SearchPattern::Regex(_)));
        assert_eq!(pattern.pattern_type(), "正则搜索");
    }

    /// 测试目标：验证无效的正则表达式能正确报错
    #[test]
    fn test_invalid_regex_pattern() {
        let result = SearchPattern::from_pattern("[invalid(regex", true);
        assert!(result.is_err());
        if let Err(SearchError::RegexCompile(msg)) = result {
            assert!(!msg.is_empty());
        } else {
            panic!("Expected RegexCompile error");
        }
    }

    /// 测试目标：验证字面量匹配逻辑
    #[test]
    fn test_literal_pattern_matching() {
        let pattern = SearchPattern::from_pattern("test", false).unwrap();
        assert!(pattern.is_match("this is a test string"));
        assert!(!pattern.is_match("this is a string"));
    }

    /// 测试目标：验证正则匹配逻辑（数字）
    #[test]
    fn test_regex_pattern_matching_digits() {
        let pattern = SearchPattern::from_pattern(r"\d+", true).unwrap();
        assert!(pattern.is_match("error code 404"));
        assert!(!pattern.is_match("no numbers here"));
    }

    /// 测试目标：验证正则匹配逻辑（交替）
    #[test]
    fn test_regex_pattern_matching_alternation() {
        let pattern = SearchPattern::from_pattern("error|warn", true).unwrap();
        assert!(pattern.is_match("error occurred"));
        assert!(pattern.is_match("warning message"));
        assert!(!pattern.is_match("info message"));
    }

    /// 测试目标：验证在单行文件中搜索字面量
    #[test]
    fn test_search_literal_found_single_line() {
        let dir = TempDir::new().unwrap();
        let file_path = create_temp_file(&dir, "hello world");
        let pattern = SearchPattern::from_pattern("hello", false).unwrap();

        let result = search_in_file(&file_path, &pattern);
        assert!(result.is_ok());
    }

    /// 测试目标：验证在多行文件中搜索正则表达式
    #[test]
    fn test_search_regex_found_multiline() {
        let dir = TempDir::new().unwrap();
        let content = "line 1\nerror 404\nline 3\nerror 500";
        let file_path = create_temp_file(&dir, content);
        let pattern = SearchPattern::from_pattern(r"error \d+", true).unwrap();

        let result = search_in_file(&file_path, &pattern);
        assert!(result.is_ok());
    }

    /// 测试目标：验证搜索不存在的关键词
    #[test]
    fn test_search_literal_not_found() {
        let dir = TempDir::new().unwrap();
        let file_path = create_temp_file(&dir, "hello world");
        let pattern = SearchPattern::from_pattern("xyz", false).unwrap();

        let result = search_in_file(&file_path, &pattern);
        assert!(result.is_ok());
    }

    /// 测试目标：验证搜索不存在文件时返回错误
    #[test]
    fn test_search_file_not_found() {
        let pattern = SearchPattern::from_pattern("test", false).unwrap();
        let result = search_in_file("/nonexistent/file.txt", &pattern);
        assert!(result.is_err());
    }
}
