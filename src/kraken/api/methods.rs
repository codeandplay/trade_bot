pub enum Method {
    Balance,
    Time,
    Assets,
    TradesHistory,
    QueryOrders,
    OHLC,
}

impl From<Method> for &str {
    fn from(m: Method) -> Self {
        match m {
            Method::Balance => "Balance",
            Method::Time => "Time",
            Method::Assets => "Assets",
            Method::TradesHistory => "TradesHistory",
            Method::QueryOrders => "QueryOrders",
            Method::OHLC => "OHLC",
        }
    }
}
