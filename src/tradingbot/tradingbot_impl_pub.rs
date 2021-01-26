use std::error::Error;

use log::{debug, info, trace};

use super::{
    market::Market,
    tradingbot::{TradingBot, TradingConfig},
};

impl TradingBot {
    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Try to place order");

        trace!("Getting balances");
        let balance = self.market.get_balances().await?;
        trace!("balance is {}", balance);

        trace!("Getting market price");
        let balance = self.market.get_market_price().await?;

        self.try_to_buy().await?;

        Ok(())
    }

    pub fn new(trading_config: TradingConfig, market: Box<dyn Market>) -> Self {
        TradingBot {
            trading_config,
            market,
        }
    }

    async fn try_to_buy(&mut self) -> Result<f32, Box<dyn Error>> {
        info!("try to buy");
        Ok(1.0)
    }

    async fn try_to_sell(&mut self) -> Result<f32, Box<dyn Error>> {
        info!("try to sell");
        Ok(1.0)
    }
}
