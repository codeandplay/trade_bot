/// Kraken API implementation
use crate::kraken::api::utils::create_signature;
use chrono::Utc;
use data_encoding::BASE64;
use log::trace;
use reqwest::blocking::Response;
use reqwest::header::USER_AGENT;
use serde::de::DeserializeOwned;
use std::{collections::HashMap, error::Error};

use reqwest::header::HeaderMap;

use super::{api::KrakenAPI, error::KrakenError, methods::Method, types::KrakenResponse};

const API_URL: &str = "https://api.kraken.com";
const API_VERSION: &str = "0";
const API_USER_AGENT: &str = "Kraken Rust API Agent";
pub const BTCUSD: &str = "XXBTZUSD";

/// Kraken API
impl KrakenAPI {
    pub fn new(api_key: String, secret: String) -> KrakenAPI {
        KrakenAPI {
            api_key,
            secret,
            client: reqwest::blocking::Client::new(),
        }
    }

    /// query private endpoints
    pub async fn query_private<T>(
        &self,
        method: Method,
        params: &mut HashMap<String, String>,
    ) -> Result<KrakenResponse<T>, Box<dyn Error>>
    where
        T: DeserializeOwned + 'static,
    {
        let method: &str = method.into();
        let url_path = format!("/{}/private/{}", API_VERSION, method);
        let url = format!("{}{}", API_URL, url_path);
        let secret_bytes = BASE64
            .decode(&self.secret.as_bytes())
            .expect("Not able to decode Kraken api secret");
        let nonce = format!("{}", Utc::now().timestamp_millis() * 1000);
        params.insert("nonce".to_owned(), nonce.to_owned());

        let sig = create_signature(&url_path, params, &secret_bytes)?;

        let mut header_map = HeaderMap::new();
        header_map.insert(
            "API-Key",
            self.api_key.parse().expect("fail to parse api key"),
        );

        header_map.insert(
            "API-Sign",
            sig.parse().expect("fail to parse request signature"),
        );

        trace!("Query request url: {}", url);
        trace!("Query request method: {}", method);
        trace!("Query with nonce: {}", nonce);
        trace!("Query request params: {:?}", params);

        let res: KrakenResponse<T> = self.do_request(&url, params, &header_map).await?.json()?;
        Ok(res)
    }

    /// query public endpoints
    pub async fn query_public<T>(
        &self,
        method: Method,
        params: &HashMap<String, String>,
    ) -> Result<KrakenResponse<T>, Box<dyn Error>>
    where
        T: DeserializeOwned + 'static,
    {
        let method: &str = method.into();
        let url = format!("{}/{}/public/{}", API_URL, API_VERSION, method);

        trace!("Query request url: {}", url);
        trace!("Query request method: {}", method);

        let res: KrakenResponse<T> = self
            .do_request(&url, params, &HeaderMap::new())
            .await?
            .json()?;
        Ok(res)
    }

    /// Send Http reqeust.
    async fn do_request(
        &self,
        url: &str,
        params: &HashMap<String, String>,
        headers: &HeaderMap,
    ) -> Result<Response, Box<dyn Error>> {
        let mut header_map = HeaderMap::new();
        header_map.insert(USER_AGENT, API_USER_AGENT.parse().unwrap());

        // Additional headers.
        for (n, v) in headers {
            header_map.insert(n, v.to_owned());
        }

        let res = self
            .client
            .post(url)
            .form(params)
            .headers(header_map)
            .send()?;

        let ok = res.status().is_success();

        if !ok {
            let status_code = res.status();
            let res: KrakenResponse<serde_json::Value> = res.json()?;
            return Err(Box::new(KrakenError::new(Some(status_code), res.error)));
        }

        Ok(res)
    }
}
