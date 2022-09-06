use std::collections::HashMap;
pub mod casbin_proto {
    tonic::include_proto!("proto");
}
pub mod datastructure;
pub mod proto;
pub mod server;
use casbin::{Adapter, CachedEnforcer};
use futures::lock::Mutex;
use std::cell::RefCell;
use std::sync::Arc;

use crate::datastructure::hashtable::HashTable;
// Arc is used to share data betweeen the threads, threads in rust?

#[derive(Default)]
pub struct CasbinGRPC {
    enforcer_map: HashMap<i32, Arc<Mutex<CachedEnforcer>>>,
    // use RefCell and check if thread safe, failed
    // use mutex+ arc
    // something similar to RefCell but thread safe
    // drop mutex guard
    // Atomic
    // RefCell with Arc, failed
    // Arc+Rwlock
    // HashBrown
    adapter_map: HashMap<i32, Arc<Mutex<Box<dyn Adapter>>>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: String = "[::1]:50051".parse().unwrap();
    let casbin = CasbinGRPC::default();
    // let mut hash = HashTable::<i32, Box<dyn Adapter>>::new();

    println!("Server listening on: {}", addr);
    //Server::builder()
    //    .add_service(CasbinServer::new(casbin))
    //    .serve(addr)
    //    .await?;

    Ok(())
}
