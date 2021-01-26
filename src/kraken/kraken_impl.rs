use super::{
    api::{
        api::KrakenAPI,
        error::KrakenError,
        methods::Method,
        types::{AssetInfo, KrakenResponse, ServerTime},
    },
    kraken::Kraken,
};
use crate::tradingbot::market::Market;
use async_trait::async_trait;
use chrono::DateTime;
use chrono::{TimeZone, Utc};
use log::info;
use rand::Rng;
use std::collections::HashMap;
use std::error::Error;

impl Kraken {
    pub fn new(api_key: &str, secret: &str) -> Self {
        Kraken {
            api_key: api_key.to_owned(),
            secret: secret.to_owned(),
            api_client: KrakenAPI::new(api_key.to_string(), secret.to_string()),
        }
    }

    async fn example_calls(&self) -> Result<(), Box<dyn Error>> {
        let api = KrakenAPI::new(self.api_key.clone(), self.secret.clone());

        let res = api
            .query_public::<ServerTime>(Method::Time, &HashMap::new())
            .await?;
        info!("Server time: {:?}", res);

        let mut params = HashMap::new();
        params.insert("asset".to_owned(), "ADA".to_owned());
        let res = api
            .query_public::<HashMap<String, AssetInfo>>(Method::Assets, &params)
            .await?;
        info!("asset info: {:?}", res);

        let res = api
            .query_private::<HashMap<String, String>>(Method::Balance, &mut HashMap::new())
            .await?;
        info!("Balance: {:?}", res);

        let mut params = HashMap::new();
        params.insert(
            "txid".to_owned(),
            "OLQFEY-GHLWA-4G3MPQ,OHLGNG-7VK5I-3NDXCY".to_string(),
        );
        let res = api
            .query_private::<HashMap<String, serde_json::Value>>(Method::QueryOrders, &mut params)
            .await?;
        info!("Query order: {:?}", res.result.unwrap().len());
        Ok(())
    }
}

/// Kraken Market implement  for Kraken
#[async_trait(?Send)]
impl Market for Kraken {
    async fn get_balances(&self) -> Result<f32, Box<dyn Error>> {
        let res: KrakenResponse<HashMap<String, String>> = self
            .api_client
            .query_private::<HashMap<String, String>>(Method::Balance, &mut HashMap::new())
            .await?;

        let result = res.result.expect("Balance map return from api");

        if let Some(b) = result.get("ZUSD") {
            Ok(b.parse::<f32>().expect("Balance should be number"))
        } else {
            Ok(0.0)
        }
    }

    async fn get_market_price(&self) -> Result<f32, Box<dyn Error>> {
        // https://api.kraken.com/0/public/OHLC?pair=TBTCUSD&interval=60&since=1607023200
        let mut params = HashMap::new();
        params.insert("pair".into(), "TBTCUSD".into());
        params.insert("interval".into(), "240".into());
        let res = self
            .api_client
            .query_public::<HashMap<String, serde_json::Value>>(Method::OHLC, &params)
            .await?;

        let result = res.result.expect("Should have result");

        let data = result
            .get("TBTCUSD")
            .expect("OHLC data exist for the queried pair");

        let data = data.as_array().expect("should in array of OHLC");
        let length = data.len();
        let data = &data[length - 2..length];

        let ohlc: (chrono::DateTime<chrono::Utc>, f32, f32, f32, f32, f32, u64) = data.iter().map(|val| {
            let val = val.as_array()
            .expect("OHLC should in array format: [1609027200,\"26560.5\",\"26560.5\",\"26560.5\",\"26560.5\",\"0.0\",\"0.00000000\",0]");

            let epoch = val
                .get(0)
                .expect("has epoch")
                .as_i64()
                .expect("should be int");
            let open = val
                .get(1)
                .expect("has open")
                .as_str()
                .expect("should be string")
                .parse::<f32>()
                .expect("should be float");
            let high = val
                .get(2)
                .expect("has high")
                .as_str()
                .expect("should be string")
                .parse::<f32>()
                .expect("should be float");
            let low = val
                .get(3)
                .expect("has low")
                .as_str()
                .expect("should be string")
                .parse::<f32>()
                .expect("should be float");
            let close = val
                .get(4)
                .expect("has close")
                .as_str()
                .expect("should be string")
                .parse::<f32>()
                .expect("should be float");
            let vwap = val
                .get(6)
                .expect("has vwap")
                .as_str()
                .expect("should be string")
                .parse::<f32>()
                .expect("should be float");
            let volume = val
                .get(6)
                .expect("has vwap")
                .as_str()
                .expect("should be string")
                .parse::<f32>()
                .expect("should be float");
            let count = val
                .get(7)
                .expect("has count")
                .as_u64()
                .expect("should be int");

            let timestamp = Utc.timestamp(epoch, 0);

            let ohlc = (timestamp, open, high, low, close, volume, count);
            ohlc
        }).next().expect("OHLC should exit");

        info!("latest OHLC is {:?}", ohlc);

        Ok(0.0)
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
