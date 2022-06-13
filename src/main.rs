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
use reqwest::Url;

#[tokio::main]
async fn main() -> ESDumpResult<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let args = cli::init_args();
    info!("Service Started...");

    // //unwrap is here fine since all the values are non-optional
    let es_url = args.value_of("es_url").unwrap().to_string();

    // parsing url
    let url = Url::parse(&es_url)?;
    let host = url.host().ok_or_else(|| {
        errors::ESDumpError::CustomError("unable to get host, check URL".to_string())
    })?;
    let schema = url.scheme();
    let es_url = format!("{}://{}", schema, host);
    let index = url.path().replace('/', "");

    //unwrap is here fine since all the values are non-optional
    let es_dump = ESDump {
        es_url,
        index,
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
