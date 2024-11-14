use bytes::Bytes;
use http::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;
use url::Url;

use reqwest::{header::HeaderValue, header::ACCEPT, header::CONTENT_TYPE};

pub struct RestClient {
    address: String,
    user: String,
    password: String,
}

pub struct RestConfig {
    pub address: String,
    pub username: String,
    pub password: String,
}

#[derive(Error, Debug)]
pub enum RestError {
    #[error("{0}")]
    Internal(String),
    #[error("{0}")]
    Json(String),
    #[error("{0}")]
    Http(String),
    #[error("'{0}' not found")]
    NotFound(String),
    #[error("failed to auth '{0}'")]
    AuthFailure(String),
    #[error("invalid configuration '{0}'")]
    InvalidConfig(String),
}

impl From<reqwest::Error> for RestError {
    fn from(value: reqwest::Error) -> Self {
        tracing::debug!("{:?}", value);
        RestError::Http(value.to_string())
    }
}

impl From<serde_json::Error> for RestError {
    fn from(value: serde_json::Error) -> Self {
        tracing::debug!("{:?}", value);
        RestError::Json(value.to_string())
    }
}

impl From<std::io::Error> for RestError {
    fn from(value: std::io::Error) -> Self {
        RestError::Http(value.to_string())
    }
}

impl RestClient {
    pub fn new(config: &RestConfig) -> Result<Self, RestError> {
        let url = Url::parse(&config.address)
            .map_err(|_| RestError::InvalidConfig("invalid BMC url".to_string()))?;
        let host = url
            .host_str()
            .ok_or(RestError::InvalidConfig("invalid BMC host".to_string()))?;
        let port = url.port().unwrap_or(443);
        let address = format!("{}:{}", host, port);

        Ok(RestClient {
            address,
            user: config.username.clone(),
            password: config.password.clone(),
        })
    }

    pub async fn get(&self, path: &str) -> Result<String, RestError> {
        let body = self.execute_request(Method::GET, path, None).await?;

        Ok(body)
    }

    pub async fn put(&self, path: &str, o: String) -> Result<String, RestError> {
        let body = self.execute_request(Method::PUT, path, Some(o)).await?;

        Ok(body)
    }

    pub async fn delete(&self, path: &str) -> Result<String, RestError> {
        let body = self.execute_request(Method::DELETE, path, None).await?;

        Ok(body)
    }

    pub async fn patch(&self, path: &str, o: String) -> Result<String, RestError> {
        let body = self.execute_request(Method::PATCH, path, Some(o)).await?;

        Ok(body)
    }

    async fn execute_request(
        &self,
        method: Method,
        path: &str,
        data: Option<String>,
    ) -> Result<String, RestError> {
        let schema = "https";
        let url = format!("{}://{}/{}", schema, self.address, path.trim_matches('/'));

        let body = Bytes::from(data.clone().unwrap_or(String::new()));
        tracing::debug!(
            "Method: {method}, URL: {url}, Auth: <{0}/{1}>, Body: <{2}>",
            self.user,
            self.password,
            data.unwrap_or(String::new())
        );

        let client = reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()?;
        let req = client
            .request(method, url)
            .header(ACCEPT, HeaderValue::from_static("application/json"))
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(body)
            .basic_auth(&self.user, Some(self.password.clone()))
            .build()?;
        let resp = client.execute(req).await?;

        Ok(resp.text().await?)
    }
}
