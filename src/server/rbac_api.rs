use casbin_proto::casbin_server::{Casbin, CasbinServer};
use tonic::{Request, Response, Status};

pub mod casbin_proto {
    tonic::include_proto!("proto");
}

use crate::server::enforcer::Server;
//use crate::CasbinGRPC;

impl Server {
    fn GetRolesForUser(
        &self,
        request: Request<casbin_proto::UserRoleRequest>,
    ) -> Result<Response<casbin_proto::ArrayReply>, Status> {
        println!("Received request from {:?}", request);
        let response = casbin_proto::ArrayReply {};
        Ok(Response::new(response))
    }

    async fn get_implicit_roles_for_user(
        &self,
        request: Request<casbin_proto::UserRoleRequest>,
    ) -> Result<Response<casbin_proto::ArrayReply>, Status> {
        let response = casbin_proto::ArrayReply {};
        Ok(Response::new(response))
    }
}
