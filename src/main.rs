use std::collections::HashMap;
pub mod casbin_proto {
    tonic::include_proto!("proto");
}
pub mod proto;
pub mod server;
use casbin::{Adapter, Enforcer};

#[derive(Default)]
pub struct CasbinGRPC {
    enforcer_map: HashMap<i32, Enforcer>,
    adapter_map: HashMap<i32, Box<dyn Adapter>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: String = "[::1]:50051".parse().unwrap();
    let casbin = CasbinGRPC::default();

    println!("Server listening on: {}", addr);
    //Server::builder()
    //    .add_service(CasbinServer::new(casbin))
    //    .serve(addr)
    //    .await?;

    Ok(())
}
