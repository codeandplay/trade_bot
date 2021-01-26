use super::api::api::KrakenAPI;

/// Kraken Struct
pub struct Kraken {
    pub api_key: String,
    pub secret: String,
    pub api_client: KrakenAPI,
}
