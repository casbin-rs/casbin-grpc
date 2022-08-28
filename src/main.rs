use std::collections::HashMap;
pub mod casbin_proto {
    tonic::include_proto!("proto");
}
pub mod proto;
pub mod server;
use casbin::{Adapter, Enforcer, CachedEnforcer};
use tokio::sync::RwLock;
use std::sync::Arc;
use futures::lock::Mutex;
use std::cell::RefCell;
use atomic_refcell::AtomicRefCell;

// Arc is used to share data betweeen the threads, threads in rust?

#[derive(Default)]
pub struct CasbinGRPC {
    // enforcer_map: HashMap<i32, Enforcer>,
    // enforcer_map: HashMap<i32, RefCell<CachedEnforcer>>,
    enforcer_map: HashMap<i32, Arc<Mutex<CachedEnforcer>>>,
    // enforcer_map: HashMap<i32, CachedEnforcer>,
    // enforcer_map: HashMap<i32, AtomicRefCell<CachedEnforcer>>,
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
