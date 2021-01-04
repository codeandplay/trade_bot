use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct KrakenResponse<T> {
    pub error: Vec<String>,
    pub result: Option<T>,
}

#[derive(Deserialize, Debug)]
pub struct ServerTime {
    pub unixtime: u32,
}

#[derive(Deserialize, Debug)]
pub struct AssetInfo {
    pub altname: String,
    pub aclass: String,
    pub decimals: u32,
    pub display_decimals: u32,
}
