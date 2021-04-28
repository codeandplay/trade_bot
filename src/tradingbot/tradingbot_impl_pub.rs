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

        Ok(())
    }

    pub fn new(trading_config: TradingConfig, market: Box<dyn Market>) -> Self {
        TradingBot {
            trading_config,
            market,
        }
    }

    async fn buy_order_enter(&mut self) -> Result<f32, Box<dyn Error>> {
        info!("buy order enter");
        Ok(1.0)
    }

    async fn buy_order_exit_profit(&mut self) -> Result<f32, Box<dyn Error>> {
        info!("buy order exit with profit");
        Ok(1.0)
    }

    async fn buy_order_exit_loss(&mut self) -> Result<f32, Box<dyn Error>> {
        info!("buy order exit with loss");
        Ok(1.0)
    }

    async fn sell_order_enter(&mut self) -> Result<f32, Box<dyn Error>> {
        info!("sell order enter");
        Ok(1.0)
    }

    async fn sell_order_exit_profit(&mut self) -> Result<f32, Box<dyn Error>> {
        info!("sell order exit with profit");
        Ok(1.0)
    }

    async fn sell_order_exit_loss(&mut self) -> Result<f32, Box<dyn Error>> {
        info!("sell order exit with loss");
        Ok(1.0)
    }
}
