use std::collections::HashMap;
pub mod casbin_proto {
    tonic::include_proto!("proto");
}
pub mod server;
use casbin::{Adapter, CachedEnforcer};

use crate::casbin_proto::casbin_server::CasbinServer;
use futures::lock::Mutex;
use std::sync::Arc;
use tonic::transport::Server;

// Arc is used to share data betweeen the threads, threads in rust?

#[derive(Default)]
pub struct CasbinGRPC {
    enforcer_map: HashMap<i32, Arc<Mutex<CachedEnforcer>>>,
    adapter_map: HashMap<i32, Box<dyn Adapter>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let casbin = CasbinGRPC::default();

    println!("Server listening on: {}", addr);
    Server::builder()
        .add_service(CasbinServer::new(casbin))
        .serve(addr)
        .await?;

    Ok(())
}
