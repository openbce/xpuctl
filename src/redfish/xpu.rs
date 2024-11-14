use crate::types::{Context, BMC};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{fmt, io, sync::Arc};

use super::{bluefield::Bluefield, Redfish, RedfishError};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum XPUStatus {
    Ready,
    Error,
    Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BMCVersion {
    pub description: String,
    pub id: String,
    pub version: String,
}

pub struct XPU {
    // The redfish client of XPU
    redfish: Box<dyn Redfish>,

    // The basic info of XPU
    pub vendor: String,
    pub serial_number: String,
    pub firmware_version: String,
    pub bmc: BMC,
    pub bmc_version: String,

    // The status of XPU
    pub status: XPUStatus,
}

impl ToString for XPUStatus {
    fn to_string(&self) -> String {
        match self {
            XPUStatus::Error => "Error".to_string(),
            XPUStatus::Ready => "Ready".to_string(),
            XPUStatus::Unknown => "Unknown".to_string(),
        }
    }
}

impl XPU {
    pub async fn new(bmc: &BMC) -> Result<Self, RedfishError> {
        let redfish: Box<Bluefield> = Box::new(Bluefield::new(bmc)?);
        let bmc_ver = redfish.bmc_version().await?;

        let xpu = XPU {
            redfish,
            vendor: bmc.vendor.clone(),
            serial_number: "-".to_string(),
            firmware_version: "-".to_string(),
            bmc_version: bmc_ver.version,
            bmc: bmc.clone(),
            status: XPUStatus::Ready,
        };

        Ok(xpu)
    }
}

pub async fn discover(bmc: &BMC) -> Result<(), RedfishError> {
    let redfish = Box::new(Bluefield::new(bmc)?);

    if redfish.bmc_version().await.is_ok() {
        return Ok(());
    }

    // Try to change the default password.
    let default_bmc = Bluefield::default_bmc(&bmc.name, &bmc.address);
    let default_redfish = Box::new(Bluefield::new(&default_bmc)?);
    default_redfish
        .change_password(bmc.password.clone().unwrap())
        .await?;

    // Retry BMC version by the password.
    let _ = redfish.bmc_version().await?;

    Ok(())
}
