use chrono::prelude::*;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use dotenv::dotenv;
use log::{info, warn};
use std::env;
use tokio::time;
use tokio::time::Instant;
use trade_bot::{self, Kraken, TradingBot, TradingConfig};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("api_key")
                .short("k")
                .long("api_key")
                .value_name("API_KEY")
                .help("Set API_KEY. Can also set with env variable: API_KEY. This commandline argument take precedence")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("api_secret")
                .short("s")
                .long("api_secret")
                .value_name("API_SECRET")
                .help("Set API_SECRET. Can also set with env variable: API_SECRET. This commandline argument take precedence")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("trading_cadence")
                .short("c")
                .long("trading_cadence")
                .value_name("TRADIND_CADENCE")
                .help("Set TRADIND_CADENCE. Can also set with env variable: TRADIND_CADENCE. This commandline argument take precedence")
                .takes_value(true),
        )
        .get_matches();

    env_logger::init();

    let api_key = matches
        .value_of("api_key")
        .map(|s| s.to_owned())
        .or(env::var("API_KEY").ok());

    let api_secret = matches
        .value_of("api_secret")
        .map(|s| s.to_owned())
        .or(env::var("API_SECRET").ok());

    let trading_cadence = matches
        .value_of("trading_cadence")
        .map(|s| s.to_owned())
        .or(env::var("TRADIND_CADENCE").ok())
        .unwrap_or_else(|| "10".to_owned())
        .parse::<u64>()
        .unwrap();

    if api_key.is_none() || api_secret.is_none() {
        println!("API_KEY and API_SECRET are required");
        std::process::exit(1);
    }

    let config = TradingConfig {};
    let kraken = Kraken::new(&api_key.unwrap(), &api_secret.unwrap());
    // intialize the TradingBot for kraken context
    let mut kraken_bot = TradingBot::new(config, Box::new(kraken));

    let mut interval = time::interval(time::Duration::from_secs(trading_cadence));

    loop {
        // wait every 20s
        interval.tick().await;

        // trading start time
        let start = Instant::now();
        let now: Date<Local> = Local::now().date();

        // trading kick off
        warn!("[TRADE] start at {:?}", now);
        kraken_bot
            .start()
            .await
            .expect("Fail to start a trade process");

        // trading end time
        let duration = start.elapsed();
        info!("[TRADE] end elapsed : {:?}", duration);
        info!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
    }
}
