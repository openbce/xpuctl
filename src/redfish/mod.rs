use crate::types::{Context, BMC};
use async_trait::async_trait;
use rest::RestError;
use serde::{Deserialize, Serialize};
use std::{fmt, io, sync::Arc};
use thiserror::Error;

mod bluefield;
mod rest;
mod xpu;

pub use xpu::{discover, BMCVersion, XPU};

#[async_trait]
trait Redfish {
    async fn change_password(&self, passwd: String) -> Result<(), RedfishError>;
    async fn bmc_version(&self) -> Result<BMCVersion, RedfishError>;
}

#[derive(Error, Debug)]
pub enum RedfishError {
    #[error("{0}")]
    RestError(String),
    #[error("{0}")]
    IOError(String),
}

impl From<RestError> for RedfishError {
    fn from(value: RestError) -> Self {
        RedfishError::RestError(value.to_string())
    }
}

impl From<std::io::Error> for RedfishError {
    fn from(value: std::io::Error) -> Self {
        RedfishError::IOError(value.to_string())
    }
}
