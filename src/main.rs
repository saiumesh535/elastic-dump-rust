mod api;
mod cli;
mod errors;
mod es_dump;
mod sign;
mod types;
use env_logger::Env;
use errors::ESDumpResult;
use es_dump::ESDump;
use log::{error, info};

#[tokio::main]
async fn main() -> ESDumpResult<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    info!("Service Started...");

    let args = cli::init_args();
    
    //unwrap is here fine since all the values are non-optional
    let es_dump = ESDump {
        es_url: args.value_of("es_url").unwrap().to_string(),
        index: args.value_of("index").unwrap().to_string(),
        query: args.value_of("query").unwrap().to_string(),
        file_name: args.value_of("file_name").unwrap().to_string(),
        region: args.value_of("region").unwrap().to_string(),
        key_id: args.value_of("key_id").unwrap().to_string(),
        secret_key: args.value_of("secret_key").unwrap().to_string(),
    };

    let result = ESDump::new(es_dump).dump().await;
    if result.is_err() {
        error!("{}", result.err().unwrap());
    }
    Ok(())
}
