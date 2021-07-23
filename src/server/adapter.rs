use crate::casbin_proto::NewAdapterRequest;
use regex::Regex;
use serde_json;
use tokio::fs::File;
use tonic::Request;

use serde::{Deserialize, Serialize};

static ERR_DRIVER_NAME: &str = "currently supported DriverName: file | mysql | postgres | mssql";

pub async fn check_local_config(i: &mut Request<NewAdapterRequest>) {
    let cfg: Config = load_configuration("config/connection_config.json")
        .await
        .expect("connection_config.json file not found");
    let x = i.get_mut();
    if x.connect_string.is_empty() || x.driver_name == "" {
        x.driver_name = cfg.driver;
        x.connect_string = cfg.connection;
        x.db_specified = cfg.db_specified;
    }
}

pub async fn load_configuration(file: &str) -> Result<Config, std::io::Error> {
    //Loads a default config from adapter_config in case a custom adapter isn't provided by the client.
    //DriverName, ConnectionString, and dbSpecified can be configured in the file. Defaults to 'file' mode.

    let config_file = File::open(file).await.expect("file not found");
    let decoder: Config = serde_json::from_str(file).expect("JSON was not well-formatted");
    let mut config: Config = Config::default();
    let re = Regex::new(r"\$\b((\w*))\b");
    // config.connection
    Ok(config)
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Config {
    driver: String,
    connection: String,
    enforcer: String,
    db_specified: bool,
}
