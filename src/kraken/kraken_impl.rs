use std::collections::HashMap;
use std::error::Error;

use crate::tradingbot::market::Market;

use super::{
    api::{
        api::KrakenAPI,
        methods::Method,
        types::{AssetInfo, ServerTime},
    },
    kraken::Kraken,
};

use async_trait::async_trait;
use log::info;
use rand::Rng;

impl Kraken {
    pub fn new(api_key: &str, secret: &str) -> Self {
        Kraken {
            api_key: api_key.to_owned(),
            secret: secret.to_owned(),
        }
    }
}

#[async_trait(?Send)]
impl Market for Kraken {
    async fn get_balances(&self) -> Result<f32, Box<dyn Error>> {
        let api = KrakenAPI::new(self.api_key.clone(), self.secret.clone());
        let res = api
            .query_public::<ServerTime>(Method::Time, &HashMap::new())
            .await?;
        info!("Server time: {:?}", res);
        let res = api
            .query_public::<HashMap<String, AssetInfo>>(Method::Assets, &HashMap::new())
            .await?;
        info!("Server time: {:?}", res);
        let res = api
            .query_private::<HashMap<String, String>>(Method::Balance, &mut HashMap::new())
            .await?;
        info!("Server time: {:?}", res);
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
