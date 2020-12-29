use std::error::Error;

use crate::tradingbot::market::Market;

use super::coinbase::Coinbase;

use async_trait::async_trait;
use rand::Rng;

#[async_trait(?Send)]
impl Market for Coinbase {
    async fn get_balances(&self) -> Result<f32, Box<dyn Error>> {
        // coinbase API call
        Ok(1.0)
    }

    async fn get_market_price(&self) -> Result<f32, Box<dyn Error>> {
        // coinbase API call
        let mut rng = rand::thread_rng();
        Ok(rng.gen_range(0.0..10.0))
    }

    async fn place_sell_order(&self, amount: f32) -> Result<f32, Box<dyn Error>> {
        // coinbase API call
        Ok(amount)
    }

    async fn place_buy_order(&self, amount: f32) -> Result<f32, Box<dyn Error>> {
        // coinbase API call
        Ok(amount)
    }
}
