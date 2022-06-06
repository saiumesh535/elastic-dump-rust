mod api;
mod cli;
mod errors;
mod es_dump;
mod sign;
mod types;
use env_logger::Env;
use errors::ESDumpResult;
use es_dump::ESDump;
use log::{info, error};

#[tokio::main]
async fn main() -> ESDumpResult<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    info!("Service Started...");
    let result = ESDump::new().dump().await;
    if result.is_err() {
        error!("{}", result.err().unwrap());
    }
    Ok(())
}
