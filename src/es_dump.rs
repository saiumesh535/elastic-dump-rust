use std::fs::File;
use std::io::Write;

use crate::api;
use crate::cli;
use crate::errors::ESDumpResult;
use crate::sign;
use crate::types;
use log::info;
use serde_json::{from_str, to_string};

#[derive(Debug)]
pub struct ESDump {
    pub es_url: String,
    pub index: String,
    pub file_name: String,
    pub key_id: String,
    pub secret_key: String,
    pub query: String,
    pub region: String,
}

impl ESDump {
    pub fn new() -> Self {
        //unwrap is here fine since all the values are non-optional
        let args = cli::init_args();
        ESDump {
            es_url: args.value_of("es_url").unwrap().to_string(),
            index: args.value_of("index").unwrap().to_string(),
            query: args.value_of("query").unwrap().to_string(),
            file_name: args.value_of("file_name").unwrap().to_string(),
            region: args.value_of("region").unwrap().to_string(),
            key_id: args.value_of("key_id").unwrap().to_string(),
            secret_key: args.value_of("secret_key").unwrap().to_string(),
        }
    }

    pub async fn dump(&mut self) -> ESDumpResult<()> {
        let url_with_index = format!("{}/{}/_search?scroll=5m", self.es_url, self.index);

        info!("signing the request");
        let signed_request = sign::get_signed_request(self, url_with_index)?;
        let scroll_api = api::api(signed_request).await?;
        // get the scroll from response
        let scroll = from_str::<types::Scroll>(&scroll_api)?;

        if scroll.hits.hits.is_empty() {
            info!("There are no results for this query\n {}", self.query);
            return Ok(());
        }

        // create file
        info!("Creating a file with name {}", self.file_name);
        File::create(&self.file_name)?;
        let mut file_ref = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.file_name)?;

        // get the first set of value and insert into a file
        for hit in &scroll.hits.hits {
            info!("writing the data to the file {}", self.file_name);
            file_ref.write_all(to_string(hit)?.as_bytes())?;
            file_ref.write_all("\n".as_bytes())?;
        }

        // prepare next request
        let next_request = types::ScrollAPI {
            scroll: "5m".to_string(),
            scroll_id: scroll._scroll_id.unwrap(),
        };

        loop {
            // update url
            let scroll_url = format!("{}/_search/scroll", &self.es_url);
            self.query = to_string(&next_request)?;

            info!("signing the request");
            let scrolled_signed_request = sign::get_signed_request(self, scroll_url)?;
            let scroll_response = api::api(scrolled_signed_request).await?;
            let scroll = from_str::<types::Scroll>(&scroll_response)?;

            if scroll.hits.hits.is_empty() {
                info!("No more data to write");
                break;
            }

            info!("writing the data to the file {}", self.file_name);
            for hit in &scroll.hits.hits {
                file_ref.write_all(serde_json::to_string(hit)?.as_bytes())?;
                file_ref.write_all("\n".as_bytes())?;
            }
        }

        info!("dump is done");
        Ok(())
    }
}
