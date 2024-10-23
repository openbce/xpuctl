use crate::redfish::{rest, Redfish};
use crate::types::BMC;
use async_trait::async_trait;
use bytes::Bytes;
use hyper::Request;
use std::collections::HashMap;
use std::{io, sync::Arc};

use super::rest::{RestClient, RestConfig};
use super::xpu::BMCVersion;
use super::RedfishError;

pub struct Bluefield {
    rest: RestClient,
}

const DEFAULT_PASSWORD: &str = "0penBmc";
const DEFAULT_USER: &str = "root";
const VENDOR: &str = "bluefield";

#[async_trait]
impl Redfish for Bluefield {
    async fn change_password(&self, passwd: String) -> Result<(), RedfishError> {
        let data = format!("Password: {}", passwd);

        self.rest
            .patch("/redfish/v1/AccountService/Accounts/root", data)
            .await
            .map_err(RedfishError::from)?;
        Ok(())
    }

    async fn bmc_version(&self) -> Result<BMCVersion, RedfishError> {
        self.rest
            .get("redfish/v1/UpdateService/FirmwareInventory/BMC_Firmware")
            .await
            .map_err(RedfishError::from)
    }
}

impl Bluefield {
    pub fn new(bmc: &BMC) -> Result<Bluefield, RedfishError> {
        let config = RestConfig {
            address: bmc.address.clone(),
            password: bmc.password.clone().unwrap_or(DEFAULT_PASSWORD.to_string()),
            username: bmc.username.clone().unwrap_or(DEFAULT_USER.to_string()),
        };

        Ok(Bluefield {
            rest: RestClient::new(&config)?,
        })
    }

    pub fn default_bmc(name: &str, addr: &str) -> BMC {
        BMC {
            name: name.to_string(),
            address: addr.to_string(),
            vendor: VENDOR.to_string(),
            password: Some(DEFAULT_PASSWORD.to_string()),
            username: Some(DEFAULT_USER.to_string()),
        }
    }
}
