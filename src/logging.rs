use crate::config;

/// 初始化日志系统，支持 CLI 值优先、配置文件备选、默认 info
pub fn init(log_level_override: Option<&String>) {
    let runtime_log_level = log_level_override
        .cloned()
        .or_else(|| {
            config::default_config_path()
                .and_then(|path| config::load_file_config(&path).ok())
                .and_then(|cfg| cfg.log_level)
        })
        .unwrap_or_else(|| "info".to_string());

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&runtime_log_level)),
        )
        .init();
}
