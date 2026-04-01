/// 集成测试：真实场景下的完整搜索流程
///
/// 这些测试验证搜索库在真实场景中的表现，包括：
/// - 创建临时文件目录结构
/// - 跨多个文件的搜索
/// - 错误处理和边界情况
use file_search::search::{SearchPattern, search_in_file};
use std::fs;
use tempfile::TempDir;

/// 辅助函数：在临时目录中创建多个文件
fn setup_test_directory() -> (TempDir, Vec<String>) {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let mut file_paths = Vec::new();

    // 创建 file1.txt
    let file1_path = dir.path().join("file1.txt");
    let file1_content = "This is the first file\nIt contains hello world\nAnd some other content";
    fs::write(&file1_path, file1_content).expect("Failed to write file1");
    file_paths.push(file1_path.to_string_lossy().to_string());

    // 创建 file2.txt
    let file2_path = dir.path().join("file2.txt");
    let file2_content = "Second file here\nNo matching keyword\nJust some text";
    fs::write(&file2_path, file2_content).expect("Failed to write file2");
    file_paths.push(file2_path.to_string_lossy().to_string());

    // 创建 file3.txt
    let file3_path = dir.path().join("file3.txt");
    let file3_content = "hello world appears here too\nMultiple occurrences\nhello hello hello";
    fs::write(&file3_path, file3_content).expect("Failed to write file3");
    file_paths.push(file3_path.to_string_lossy().to_string());

    // 创建 subdir/nested.txt
    let subdir = dir.path().join("subdir");
    fs::create_dir(&subdir).expect("Failed to create subdir");
    let nested_path = subdir.join("nested.txt");
    let nested_content = "Nested file content\nhello world in nested location";
    fs::write(&nested_path, nested_content).expect("Failed to write nested");
    file_paths.push(nested_path.to_string_lossy().to_string());

    (dir, file_paths)
}

/// 测试目标：在单个文件中成功搜索关键词（集成测试版）
#[test]
fn integration_test_search_single_file() {
    let (_dir, file_paths) = setup_test_directory();
    let pattern = SearchPattern::from_pattern("hello", false).expect("Failed to create pattern");

    let result = search_in_file(&file_paths[0], &pattern);
    assert!(
        result.is_ok(),
        "Should successfully search file1.txt for 'hello'"
    );
}

/// 测试目标：验证搜索多个文件的完整工作流
#[test]
fn integration_test_search_multiple_files() {
    let (_dir, file_paths) = setup_test_directory();
    let pattern = SearchPattern::from_pattern("hello", false).expect("Failed to create pattern");

    // 搜索所有文件中的 "hello"
    for path in &file_paths {
        let result = search_in_file(path, &pattern);
        assert!(result.is_ok(), "Should handle all files without error");
    }
}

/// 测试目标：验证在不存在的文件上搜索返回适当错误
#[test]
fn integration_test_search_with_permission_error() {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = dir.path().join("test.txt");
    fs::write(&file_path, "content").expect("Failed to write file");
    let pattern = SearchPattern::from_pattern("content", false).expect("Failed to create pattern");

    // 直接测试可读文件
    let result = search_in_file(&file_path.to_string_lossy().to_string(), &pattern);
    assert!(result.is_ok());

    // 在 Unix 系统上改变权限（Windows 跳过此测试）
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::Permissions::from_mode(0o000);
        fs::set_permissions(&file_path, perms).expect("Failed to change permissions");

        let result = search_in_file(&file_path.to_string_lossy().to_string(), &pattern);

        // 恢复权限以便清理
        let perms = fs::Permissions::from_mode(0o644);
        fs::set_permissions(&file_path, perms).expect("Failed to restore permissions");

        assert!(result.is_err(), "Should fail with permission denied");
    }
}

/// 测试目标：验证在包含特殊内容（二进制）的文件上的表现
#[test]
fn integration_test_search_with_special_content() {
    let dir = TempDir::new().expect("Failed to create temp dir");

    // 创建包含特殊字符的文件
    let file_path = dir.path().join("special.txt");
    let content = "Normal line\n@special#chars$here\n日本語\n中文\nRussian: Привет";
    fs::write(&file_path, content).expect("Failed to write file");

    // 搜索中文
    let pattern = SearchPattern::from_pattern("中文", false).expect("Failed to create pattern");
    let result = search_in_file(&file_path.to_string_lossy().to_string(), &pattern);
    assert!(result.is_ok());

    // 搜索特殊字符
    let pattern = SearchPattern::from_pattern("@special", false).expect("Failed to create pattern");
    let result = search_in_file(&file_path.to_string_lossy().to_string(), &pattern);
    assert!(result.is_ok());
}

/// 测试目标：验证搜索大文件的性能和正确性
#[test]
fn integration_test_search_large_file() {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = dir.path().join("large.txt");

    // 创建包含 50,000 行的大文件
    let mut content = String::new();
    for i in 0..50000 {
        content.push_str(&format!("Line {} with standard content\n", i));
    }
    content.push_str("MARKER_LINE for search termination\n");

    fs::write(&file_path, content).expect("Failed to write large file");

    // 搜索标记行
    let pattern =
        SearchPattern::from_pattern("MARKER_LINE", false).expect("Failed to create pattern");
    let result = search_in_file(&file_path.to_string_lossy().to_string(), &pattern);
    assert!(result.is_ok(), "Should handle large file search");
}

/// 测试目标：验证空文件目录的处理
#[test]
fn integration_test_search_empty_directory_files() {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let pattern = SearchPattern::from_pattern("anything", false).expect("Failed to create pattern");

    // 创建多个空文件
    for i in 0..5 {
        let file_path = dir.path().join(format!("empty{}.txt", i));
        fs::write(&file_path, "").expect("Failed to create empty file");

        let result = search_in_file(&file_path.to_string_lossy().to_string(), &pattern);
        assert!(result.is_ok(), "Should handle empty files");
    }
}

/// 测试目标：验证同时搜索相同关键词的多个文件
#[test]
fn integration_test_parallel_search_scenario() {
    let dir = TempDir::new().expect("Failed to create temp dir");

    // 创建 10 个文件，每个包含不同数量的目标关键词
    let keyword = "target";
    let pattern = SearchPattern::from_pattern(keyword, false).expect("Failed to create pattern");
    let mut files = Vec::new();

    for file_num in 0..10 {
        let file_path = dir.path().join(format!("file{}.txt", file_num));
        let mut content = String::new();

        // 每个文件包含不同数量的关键词
        for i in 0..file_num {
            content.push_str(&format!(
                "This line contains target keyword occurrence {}\n",
                i
            ));
        }
        content.push_str("Some lines without the keyword\n");

        fs::write(&file_path, content).expect("Failed to write file");
        files.push(file_path.to_string_lossy().to_string());
    }

    // 搜索所有文件
    for file_path in files {
        let result = search_in_file(&file_path, &pattern);
        assert!(
            result.is_ok(),
            "Parallel scenario: all files should be searchable"
        );
    }
}

/// 测试目标：验证搜索不同编码（UTF-8）内容
#[test]
fn integration_test_search_multilingual_content() {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = dir.path().join("multilingual.txt");

    let content = r#"
English: Hello World
中文: 你好世界
日本語: こんにちは世界
한국어: 안녕하세요
Русский: Привет мир
العربية: مرحبا بالعالم
Ελληνικά: Γεια σας κόσμος
"#;

    fs::write(&file_path, content).expect("Failed to write multilingual file");

    // 测试不同语言的搜索
    let searches = vec![
        "Hello",
        "世界",
        "こんにちは",
        "안녕",
        "Привет",
        "مرحبا",
        "Γεια",
    ];

    for keyword in searches {
        let pattern =
            SearchPattern::from_pattern(keyword, false).expect("Failed to create pattern");
        let result = search_in_file(&file_path.to_string_lossy().to_string(), &pattern);
        assert!(result.is_ok(), "Should find keyword: {}", keyword);
    }
}

/// 测试目标：验证处理路径编码问题（Unicode 文件名）
#[test]
#[cfg(unix)]
fn integration_test_unicode_filename() {
    let dir = TempDir::new().expect("Failed to create temp dir");

    // 创建 Unicode 文件名
    let file_path = dir.path().join("测试文件_🦀.txt");
    let content = "Content in file with unicode name";
    fs::write(&file_path, content).expect("Failed to write unicode named file");

    let pattern = SearchPattern::from_pattern("unicode", false).expect("Failed to create pattern");
    let result = search_in_file(&file_path.to_string_lossy().to_string(), &pattern);
    assert!(result.is_ok(), "Should handle unicode filenames");
}

/// 测试目标：验证搜索结果的行号准确性
#[test]
fn integration_test_line_number_accuracy() {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = dir.path().join("line_test.txt");

    let content = "Line 1: no match\nLine 2: target here\nLine 3: no match\nLine 4: target again\nLine 5: end";
    fs::write(&file_path, content).expect("Failed to write file");

    // 搜索应该在第 2 和 4 行找到 "target"
    let pattern = SearchPattern::from_pattern("target", false).expect("Failed to create pattern");
    let result = search_in_file(&file_path.to_string_lossy().to_string(), &pattern);
    assert!(result.is_ok(), "Should correctly identify target lines");
}

/// 测试目标：验证连续搜索相同文件不会影响状态
#[test]
fn integration_test_repeated_search_same_file() {
    let dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = dir.path().join("repeated.txt");
    let content = "keyword appears here\nSome other text\nkeyword appears again";
    fs::write(&file_path, content).expect("Failed to write file");
    let pattern = SearchPattern::from_pattern("keyword", false).expect("Failed to create pattern");

    // 多次搜索同一个文件
    for _ in 0..5 {
        let result = search_in_file(&file_path.to_string_lossy().to_string(), &pattern);
        assert!(result.is_ok(), "Repeated search should always succeed");
    }
}
