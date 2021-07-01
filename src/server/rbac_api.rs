use casbin_proto::casbin_server::{Casbin, CasbinServer};
use tonic::{Request, Response, Status};

pub mod casbin_proto {
    tonic::include_proto!("proto");
}

use casbin_proto::{ArrayReply, UserRoleRequest};

//use crate::server::enforcer::Server;
use crate::CasbinGRPC;


#[tonic::async_trait]
impl Casbin for CasbinGRPC {
    // get_roles_for_user gets the roles that a user has.
    async fn get_roles_for_user(
        &self,
        request: Request<UserRoleRequest>,
    ) -> Result<Response<ArrayReply>, Status> {
        //debug!("Received request from {:?}", request);
        let x =  
        let roles_for_user = enforcer::get_enforcer(request.into_inner().enforcer_handler as i32);
        let response = ArrayReply {
            list: roles_for_user,
        };
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
