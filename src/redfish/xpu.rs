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

    // The status of XPU
    pub status: XPUStatus,
}

impl fmt::Display for XPUStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            XPUStatus::Error => "Error",
            XPUStatus::Ready => "Ready",
            XPUStatus::Unknown => "Unknown",
        };

        write!(f, "{}", s)
    }
}

impl XPU {
    pub async fn new(bmc: &BMC) -> Result<Self, RedfishError> {
        let redfish = Box::new(Bluefield::new(bmc)?);
        let bmc_ver = redfish.bmc_version().await?;

        let mut xpu = XPU {
            redfish,
            vendor: bmc.vendor.clone(),
            serial_number: "-".to_string(),
            firmware_version: "-".to_string(),
            bmc: bmc.clone(),
            status: XPUStatus::Ready,
        };

        Ok(xpu)
    }
}
