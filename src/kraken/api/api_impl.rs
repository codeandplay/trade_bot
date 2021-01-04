use crate::kraken::api::utils::create_signature;
use chrono::{Timelike, Utc};
use data_encoding::BASE64;
use log::{debug, error, trace, warn};
use reqwest::blocking::Response;
use reqwest::header::HeaderName;
use reqwest::header::USER_AGENT;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::{collections::HashMap, error::Error, fmt, io};

use reqwest::header::HeaderMap;

use super::{
    api::KrakenAPI,
    error::KrakenError,
    methods::Method,
    types::{AssetInfo, KrakenResponse, ServerTime},
};

use super::utils;

const APIURL: &str = "https://api.kraken.com";
const APIVersion: &str = "0";
const APIUserAgent: &str = "Kraken Rust API Agent";

impl KrakenAPI {
    pub fn new(api_key: String, secret: String) -> KrakenAPI {
        KrakenAPI {
            api_key,
            secret,
            client: reqwest::blocking::Client::new(),
        }
    }

    pub async fn query_private<T>(
        &self,
        method: Method,
        params: &mut HashMap<String, String>,
    ) -> Result<KrakenResponse<T>, Box<dyn Error>>
    where
        T: DeserializeOwned + 'static,
    {
        let method: &str = method.into();
        let url_path = format!("/{}/private/{}", APIVersion, method);
        let url = format!("{}{}", APIURL, url_path);
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

        debug!("Query request url: {}", url);
        debug!("Query request method: {}", method);
        debug!("Query with nonce: {}", nonce);
        debug!("Query with api: {}", self.api_key);
        debug!("Query with secret: {}", self.secret);
        debug!("Query request signature: {}", sig);
        debug!("Query request params: {:?}", params);
        debug!("Query request header: {:?}", header_map);

        let res: KrakenResponse<T> = self.do_request(&url, params, &header_map).await?.json()?;
        Ok(res)
    }

    pub async fn query_public<T>(
        &self,
        method: Method,
        params: &HashMap<String, String>,
    ) -> Result<KrakenResponse<T>, Box<dyn Error>>
    where
        T: DeserializeOwned + 'static,
    {
        let method: &str = method.into();
        let url = format!("{}/{}/public/{}", APIURL, APIVersion, method);

        let res: KrakenResponse<T> = self
            .do_request(&url, params, &HeaderMap::new())
            .await?
            .json()?;
        Ok(res)
    }

    async fn do_request(
        &self,
        url: &str,
        params: &HashMap<String, String>,
        headers: &HeaderMap,
    ) -> Result<Response, Box<dyn Error>> {
        let mut header_map = HeaderMap::new();
        header_map.insert(USER_AGENT, APIUserAgent.parse().unwrap());

        // Additional headers.
        for (n, v) in headers {
            header_map.insert(n, v.to_owned());
        }

        debug!("Do Request: Query request params: {:?}", params);
        debug!("Do Request: Query request header: {:?}", header_map);

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
