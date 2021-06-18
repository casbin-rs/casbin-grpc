use casbin::casbin_server::CasbinServer;

//use crate::casbin::casbin_server::Casbin;

use tonic::transport::Server;

pub mod casbin {
    tonic::include_proto!("proto");
}

#[derive(Debug, Default)]
pub struct CasbinStruct {}

pub trait CasbinTrait {}

#[tonic::async_trait]
impl CasbinTrait for CasbinStruct {}

// Runtime to run the server
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let casbinserver = CasbinStruct::default();
    println!("Start gRPC server...");
    Server::builder()
        .add_service(CasbinServer::new(casbinserver))
        .serve(addr)
        .await?;
}
