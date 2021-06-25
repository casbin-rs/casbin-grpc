use tonic::{transport::Server, Request, Response, Status};

use casbin_proto::casbin_server::{Casbin, CasbinServer};

pub mod casbin_proto {
    tonic::include_proto!("proto");
}

pub mod server;

use crate::server::rbac_api;

#[derive(Default, Debug)]
pub struct CasbinGRPC {}

//impl Casbin for CasbinGRPC {
//    fn get_roles_for_user(
//        &self,
//        request: Request<casbin_proto::UserRoleRequest>,
//    ) -> Result<Response<casbin_proto::ArrayReply>, Status> {
//        println!("Received request from {:?}", request);
//        let response = casbin_proto::ArrayReply {
//            message: format!("Hi"),
//        };
//
//        Ok(Response::new(response))
//    }
//}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: String = "[::1]:50051".parse().unwrap();
    let casbin = CasbinGRPC::default();

    println!("Server listening on: {}", addr);
    println!("{:?}", casbin);
    //Server::builder()
    //    .add_service(CasbinServer::new(casbin))
    //    .serve(addr)
    //    .await?;

    Ok(())
}
