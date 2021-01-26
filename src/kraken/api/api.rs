/// Krakne API Struct
pub struct KrakenAPI {
    pub api_key: String,
    pub secret: String,
    pub client: reqwest::blocking::Client,
}
