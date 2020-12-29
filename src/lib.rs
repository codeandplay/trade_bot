use log::{debug, error, info, trace, warn};

pub mod coinbase;
pub mod tradingbot;

pub use coinbase::Coinbase;
pub use tradingbot::{TradingBot, TradingConfig};

pub fn test_run() {
    trace!("tracing");
    warn!("warning");
}
