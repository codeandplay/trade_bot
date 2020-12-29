use super::market;

pub struct TradingBot {
    pub trading_config: TradingConfig,
    pub market: Box<dyn market::Market>,
}

pub struct TradingConfig {}
