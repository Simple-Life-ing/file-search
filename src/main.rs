use crate::cli::Args;
use crate::error::SearchError;

mod cli;
mod config;
mod error;
mod executor;
mod logging;
mod search;

use tracing::error;

fn main() {
    let args = Args::parse_args();

    logging::init(args.log_level.as_ref());

    if let Err(e) = run_from_args(&args) {
        error!("❌ 错误: {}", e);
        std::process::exit(1);
    }
}

fn run_from_args(args: &Args) -> Result<(), SearchError> {
    let config = config::load_configuration(args)?;
    executor::run(&config)
}
