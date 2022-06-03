mod api;
mod cli;
mod errors;
mod es_dump;
mod sign;
mod types;
use log::{info};
use errors::ESDumpResult;
use es_dump::ESDump;

#[tokio::main]
async fn main() -> ESDumpResult<()> {
    env_logger::init();
    info!("Service Started...");
    ESDump::new().dump().await?;
    Ok(())
}
