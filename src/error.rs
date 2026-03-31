use thiserror::Error;
use std::io;

/// File search 错误类型
#[derive(Error, Debug)]
pub enum SearchError {
    #[error("IO 错误: {0}")]
    Io(#[from] io::Error),

    #[error("目录遍历错误: {0}")]
    WalkDir(#[from] walkdir::Error),

    #[error("文件读取失败: {path}")]
    FileRead { path: String },

    /// 为未来功能预留
    #[allow(dead_code)]
    #[error("无效的路径: {0}")]
    InvalidPath(String),

    /// 为未来功能预留
    #[allow(dead_code)]
    #[error("无法转换路径编码: {0}")]
    PathEncoding(String),
}

/// Result 类型别名，默认错误类型为 SearchError
pub type Result<T> = std::result::Result<T, SearchError>;
