
/// NMD CLI version
pub const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub const MINIMUM_WATCHER_TIME: u64 = 5;