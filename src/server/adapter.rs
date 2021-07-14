use crate::casbin_proto;

use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};
use tonic::{Request, Response, Status};

//use crate::CasbinGRPC;
//use casbin::Adapter;
use serde::{Deserialize, Serialize};

static ERR_DRIVER_NAME: &str = "currently supported DriverName: file | mysql | postgres | mssql";

pub fn check_local_config(
    i: Request<casbin_proto::NewAdapterRequest>,
) -> Request<casbin_proto::NewAdapterRequest> {
    unimplemented!();
}

pub async fn load_configuration(file: String) -> Config {
    //Loads a default config from adapter_config in case a custom adapter isn't provided by the client.
    //DriverName, ConnectionString, and dbSpecified can be configured in the file. Defaults to 'file' mode.

    let config_file = File::open(file).await;
    //let decoder

    let config: Config = Config::default();
    config
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Config {
    driver: String,
    connection: String,
    enforcer: String,
    db_specified: bool,
}
