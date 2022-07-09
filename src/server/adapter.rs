use crate::casbin_proto::NewAdapterRequest;
use casbin::{Adapter, FileAdapter};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use sqlx_adapter::SqlxAdapter;
use tokio::fs::File;
use tonic::Request;

pub static ERR_DRIVER_NAME: &str =
    "currently supported DriverName: file | mysql | postgres | mssql";

pub async fn new_adapter(
    request: &mut Request<NewAdapterRequest>,
) -> Result<Box<dyn Adapter>, &'static str> {
    let a: Box<dyn Adapter>;
    check_local_config(request);
    let support_driver_names = vec![
        String::from("file"),
        String::from("mysql"),
        String::from("postgres"),
        String::from("mssql"),
    ];
    let connect_string = &request.get_ref().connect_string;
    if &request.get_ref().driver_name == "file" {
        a = Box::new(FileAdapter::new(connect_string.to_string()));
    } else {
        let mut support: bool = false;
        for driver_name in support_driver_names.iter() {
            if driver_name == &request.get_ref().driver_name {
                support = true;
                break;
            }
        }
        if !support {
            return Err(ERR_DRIVER_NAME);
        }
        a = Box::new(SqlxAdapter::new(connect_string, 8).await.unwrap());
    }
    Ok(a)
}

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
    let config: Config = Config::default();
    let re = Regex::new(r"\$\b((\w*))\b");

    // config.connection
    Ok(config)
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Config {
    pub driver: String,
    pub connection: String,
    pub enforcer: String,
    pub db_specified: bool,
}
