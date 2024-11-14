use crate::redfish::{RedfishError, XPU};
use crate::types::Context;

pub async fn run(cxt: &Context) -> Result<(), RedfishError> {
    println!(
        "{:<20}{:<10}{:<15}{:<10}{:<15}{:<15}{}",
        "ID", "Status", "Vendor", "FW", "SN", "BMC", "Address"
    );
    for bmc in cxt.bmc.iter() {
        let xpu = XPU::new(bmc).await?;
        println!(
            "{:<20}{:<10}{:<15}{:<10}{:<15}{:<15}{}",
            xpu.bmc.name,
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
