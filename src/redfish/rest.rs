use std::fmt;
use std::fmt::{Display, Formatter};
use url::Url;

use http_body_util::{BodyExt, Empty, Full};
use hyper::body::Bytes;
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper::http::StatusCode;
use hyper::{body::Buf, Request};
use hyper::{Method, Response};
use hyper_tls::HttpsConnector;
use hyper_util::rt::TokioIo;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use tokio::net::TcpStream;

pub struct RestClient {
    address: String,
    auth_info: String,
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

impl From<hyper::Error> for RestError {
    fn from(value: hyper::Error) -> Self {
        RestError::Http(value.to_string())
    }
}

impl From<serde_json::Error> for RestError {
    fn from(value: serde_json::Error) -> Self {
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
        let port = url.port().unwrap_or(80);
        let address = format!("{}:{}", host, port);

        Ok(RestClient {
            address,
            auth_info: "".to_string(),
        })
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, RestError> {
        let body = self.execute_request::<T>(Method::GET, path, None).await?;
        serde_json::from_reader(body.reader()).map_err(|e| RestError::Json(e.to_string()))
    }

    pub async fn put<T: DeserializeOwned + Serialize>(
        &self,
        path: &str,
        o: T,
    ) -> Result<T, RestError> {
        let input = serde_json::to_string(&o)?;
        let body = self
            .execute_request::<T>(Method::PUT, path, Some(input))
            .await?;

        serde_json::from_reader(body.reader()).map_err(|e| RestError::Json(e.to_string()))
    }

    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, RestError> {
        let body = self
            .execute_request::<T>(Method::DELETE, path, None)
            .await?;
        serde_json::from_reader(body.reader()).map_err(|e| RestError::Json(e.to_string()))
    }

    pub async fn patch<T: DeserializeOwned + Serialize>(
        &self,
        path: &str,
        o: T,
    ) -> Result<T, RestError> {
        let input = serde_json::to_string(&o)?;
        let body = self
            .execute_request::<T>(Method::PATCH, path, Some(input))
            .await?;

        serde_json::from_reader(body.reader()).map_err(|e| RestError::Json(e.to_string()))
    }

    async fn execute_request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        data: Option<String>,
    ) -> Result<Bytes, RestError> {
        let schema = "http";
        let url = format!("{}://{}/{}", schema, self.address, path.trim_matches('/'));

        let body = data.unwrap_or(String::new());

        let req = hyper::Request::builder()
            .method(method)
            .uri(url)
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, self.auth_info.to_string())
            .body(Full::<Bytes>::new(Bytes::from(body)))
            .map_err(|e| RestError::InvalidConfig(e.to_string()))?;

        let stream = TcpStream::connect(self.address.clone()).await?;
        let io = TokioIo::new(stream);

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                tracing::error!("Failed to connect to yangtze-apiserver: {:?}", err);
            }
        });

        let resp = sender.send_request(req).await?;

        if resp.status() != StatusCode::OK {
            return Err(RestError::Http(format!("{}", resp.status())));
        }

        let body = resp.collect().await?;

        Ok(body.to_bytes())
    }
}
