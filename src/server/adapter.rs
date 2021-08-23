use crate::casbin_proto::NewAdapterRequest;
use casbin::FileAdapter;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use tokio::fs::File;
use tonic::Request;

pub static ERR_DRIVER_NAME: &str =
    "currently supported DriverName: file | mysql | postgres | mssql";

pub async fn new_adapter(
    i: &mut Request<NewAdapterRequest>,
) -> Result<FileAdapter<String>, &'static str> {
    let a: FileAdapter<String>;
    check_local_config(i);
    let support_driver_names = vec![
        String::from("file"),
        String::from("mysql"),
        String::from("postgres"),
        String::from("mssql"),
    ];
    let st = String::from("file");
    let connect_string = &i.get_mut().connect_string;

    match &i.get_mut().driver_name {
        st => a = FileAdapter::new(connect_string.to_string()),
        _ => {
            let mut support: bool = false;
            for driver_name in support_driver_names.iter() {
                if driver_name == &i.get_mut().driver_name {
                    support = true;
                    break;
                }
            }
            if !support {
                return Err(ERR_DRIVER_NAME);
            }
            // a, err = gormadapter.NewAdapter(in.DriverName, in.ConnectString, in.DbSpecified)
        }
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
    let mut config: Config = Config::default();
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
