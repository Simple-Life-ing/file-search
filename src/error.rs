use std::io;
use thiserror::Error;

/// File search 错误类型
#[derive(Error, Debug)]
pub enum SearchError {
    #[error("IO 错误: {0}")]
    Io(#[from] io::Error),

    #[error("目录遍历错误: {0}")]
    WalkDir(#[from] walkdir::Error),

    #[error("Rayon 线程池错误: {0}")]
    RayonPool(#[from] rayon::ThreadPoolBuildError),

    #[error("文件读取失败: {path}")]
    FileRead { path: String },

    /// 为未来功能预留
    #[allow(dead_code)]
    #[error("无效的路径: {0}")]
    InvalidPath(String),

    #[error("缺少必需参数: {0}")]
    MissingParameter(String),

    #[error("配置文件解析错误: {0}")]
    ConfigParse(String),

    /// 为未来功能预留
    #[allow(dead_code)]
    #[error("无法转换路径编码: {0}")]
    PathEncoding(String),
}

/// Result 类型别名，默认错误类型为 SearchError
pub type Result<T> = std::result::Result<T, SearchError>;
#[cfg(test)]
mod tests {
    use super::*;

    /// 测试目标：验证 IO 错误能正确自动转换为 SearchError
    #[test]
    fn test_io_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let search_err: SearchError = io_err.into();

        let error_msg = format!("{}", search_err);
        assert!(error_msg.contains("IO 错误"));
    }

    /// 测试目标：验证文件读取错误的错误消息中包含路径信息
    #[test]
    fn test_file_read_error() {
        let err = SearchError::FileRead {
            path: "/test/path.txt".to_string(),
        };

        let error_msg = format!("{}", err);
        assert!(error_msg.contains("文件读取失败"));
        assert!(error_msg.contains("/test/path.txt"));
    }

    /// 测试目标：验证无效路径错误的错误消息正确
    #[test]
    fn test_invalid_path_error() {
        let err = SearchError::InvalidPath("invalid path reason".to_string());

        let error_msg = format!("{}", err);
        assert!(error_msg.contains("无效的路径"));
        assert!(error_msg.contains("invalid path reason"));
    }

    /// 测试目标：验证路径编码错误的错误消息正确
    #[test]
    fn test_path_encoding_error() {
        let err = SearchError::PathEncoding("encoding error".to_string());

        let error_msg = format!("{}", err);
        assert!(error_msg.contains("无法转换路径编码"));
        assert!(error_msg.contains("encoding error"));
    }

    /// 测试目标：验证错误类型的 Debug 格式化输出
    #[test]
    fn test_error_debug_format() {
        let err = SearchError::InvalidPath("test".to_string());
        let debug_msg = format!("{:?}", err);
        assert!(debug_msg.contains("InvalidPath"));
    }

    /// 测试目标：验证 Result<T> 的 Ok 分支正确处理
    #[test]
    fn test_result_ok() {
        let result: Result<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    /// 测试目标：验证 Result<T> 的 Err 分支正确处理
    #[test]
    fn test_result_err() {
        let error = SearchError::InvalidPath("test error".to_string());
        let result: Result<i32> = Err(error);
        assert!(result.is_err());
    }

    /// 测试目标：验证权限拒绝 IO 错误能正确转换
    #[test]
    fn test_io_error_permission_denied() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "permission denied");
        let search_err: SearchError = io_err.into();

        let error_msg = format!("{}", search_err);
        assert!(error_msg.contains("IO 错误"));
    }
}
