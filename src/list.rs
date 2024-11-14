use std::io;

use crate::redfish::{RedfishError, XPU};
use crate::types::Context;

pub async fn run(cxt: &Context) -> Result<(), RedfishError> {
    println!(
        "{:<5}{:<10}{:<15}{:<10}{:<15}{:<15}{}",
        "ID", "Status", "Vendor", "FW", "SN", "BMC", "Address"
    );
    for (i, bmc) in cxt.bmc.iter().enumerate() {
        let xpu = XPU::new(bmc).await?;
        println!(
            "{:<5}{:<10}{:<15}{:<10}{:<15}{:<15}{}",
            i,
            xpu.status.to_string(),
            xpu.vendor,
            xpu.firmware_version,
            xpu.serial_number,
            xpu.bmc_version,
            xpu.bmc.address,
        );
    }

    Ok(())
}
