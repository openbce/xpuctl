use crate::redfish::{self, RedfishError};
use crate::types::Context;

pub async fn run(cxt: &Context) -> Result<(), RedfishError> {
    println!("{:<20}| {:<30}| {:<50}", "Name", "BMC", "Status");
    for _ in 0..100 {
        print!("-");
    }
    println!();

    for bmc in cxt.bmc.iter() {
        match redfish::discover(bmc).await {
            Ok(_) => println!("{:<20}| {:<30}| {:<50}", bmc.name, bmc.address, "Ok"),
            Err(e) => println!(
                "{:<20}| {:<30}| {:<50}",
                bmc.name,
                bmc.address,
                e.to_string()
            ),
        }
    }

    Ok(())
}
