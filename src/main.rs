//! Kraken is a LAN-based, distributed, fault tolerant application deployment platform
//!
//! See CRATE_MANIFEST_DIR/documentation for more information

#![warn(
    missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    unused_extern_crates
)]

use log::{error, info};

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let orchestrator_ip = kraken_utils::network::find_orchestrator_on_lan(8000).await;

    match &orchestrator_ip {
        Some(ip) => {
            info!("Orchestrator detected at {}", ip);
        }
        None => {
            error!("No orchestrator on port 8000 detected on this LAN");
        }
    };

    Ok(())
}
