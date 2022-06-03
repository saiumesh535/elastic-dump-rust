mod api;
mod cli;
mod errors;
mod es_dump;
mod sign;
mod types;
use errors::ESDumpResult;
use es_dump::ESDump;
use log::info;

#[tokio::main]
async fn main() -> ESDumpResult<()> {
    env_logger::init();
    info!("Service Started...");
    ESDump::new().dump().await?;
    Ok(())
}
