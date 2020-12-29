use chrono::prelude::*;
use dotenv::dotenv;
use log::{debug, error, info, trace, warn};
use std::env;
use tokio::time;
use tokio::time::Instant;
use trade_bot::{self, Coinbase, TradingBot, TradingConfig};

#[tokio::main]
async fn main() {
    dotenv().ok();

    env_logger::init();

    let config = TradingConfig {};
    // intialize the TradingBot for coinbase context
    let mut coinbase_bot = TradingBot::new(config, Box::new(Coinbase {}));

    // set the interval for every 20s
    let trading_cadence = env::var("TRADIND_CADENCE")
        .unwrap_or_else(|_| "2".to_owned())
        .parse::<u64>()
        .unwrap();

    let mut interval = time::interval(time::Duration::from_secs(trading_cadence));

    loop {
        // wait every 20s
        interval.tick().await;

        // trading start time
        let start = Instant::now();
        let now: Date<Local> = Local::now().date();

        // trading kick off
        warn!("[TRADE] start at {:?}", now);
        coinbase_bot.start().await.unwrap();

        // trading end time
        let duration = start.elapsed();
        info!("[TRADE] end elapsed : {:?}", duration);
        info!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    }
}
