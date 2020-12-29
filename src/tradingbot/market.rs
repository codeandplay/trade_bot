use async_trait::async_trait;
use std::error::Error;

#[async_trait(?Send)]
pub trait Market {
    async fn get_balances(&self) -> Result<f32, Box<dyn Error>>;
    async fn get_market_price(&self) -> Result<f32, Box<dyn Error>>;
    async fn place_sell_order(&self, amount: f32) -> Result<f32, Box<dyn Error>>;
    async fn place_buy_order(&self, amount: f32) -> Result<f32, Box<dyn Error>>;
}
