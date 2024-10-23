use crate::redfish::{rest, Redfish};
use crate::types::BMC;
use async_trait::async_trait;
use bytes::Bytes;
use hyper::Request;
use std::{io, sync::Arc};

use super::rest::RestConfig;
use super::RedfishError;
use super::xpu::BMCVersion;

pub struct Bluefield {
    bmc: BMC,
}

#[async_trait]
impl Redfish for Bluefield {
    async fn connect(&self) -> Result<(), RedfishError> {
        let config = RestConfig {
            address: self.bmc.address.clone(),
        };

        let client = rest::RestClient::new(&config)?;

        

        todo!()
    }

    async fn change_password(&self) -> Result<(), RedfishError> {
        todo!()
    }

    async fn bmc_version(&self) -> Result<BMCVersion, RedfishError> {
        todo!()
    }
}

impl Bluefield {
    pub fn new(bmc: &BMC) -> Result<Bluefield, RedfishError> {
        let bf = Bluefield {
            bmc: (*bmc).clone(),
        };

        Ok(bf)
    }
}
