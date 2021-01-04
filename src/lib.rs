use log::{debug, error, info, trace, warn};

pub mod kraken;
pub mod tradingbot;

pub use kraken::Kraken;
pub use tradingbot::{TradingBot, TradingConfig};

pub fn test_run() {
    trace!("tracing");
    warn!("warning");
}
