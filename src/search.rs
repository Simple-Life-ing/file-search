use crate::error::{Result, SearchError};
use std::fs::File;
use std::io::{BufRead, BufReader};

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
        eprintln!("警告: 无法打开文件 {}: {}", path, e);
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
