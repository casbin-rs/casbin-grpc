use crate::casbin_proto;
use casbin_proto::casbin_server::{Casbin, CasbinServer};
use casbin_proto::{NewEnforcerReply, NewEnforcerRequest};
use std::collections::HashMap;
use tonic::{Request, Response, Status};

use crate::CasbinGRPC;
use casbin::{Adapter, Enforcer};

impl CasbinGRPC {
    pub fn new_server() -> Self {
        Self {
            enforcerMap: HashMap::new(),
            adapterMap: HashMap::new(),
        }
    }
    pub fn get_enforcer(&self, handle: i32) -> Enforcer {
        self.enforcerMap.get(&handle)
    }
    pub fn get_adapter(&self, handle: i32) -> Box<dyn Adapter> {
        self.adapterMap.get(&handle)
    }
    pub fn add_enforcer(&self, e: Enforcer) -> i32 {
        let cnt: i32 = self.enforcerMap.len() as i32;
        self.enforcerMap[&cnt] = e;
        cnt
    }
    pub fn add_adapter(&self, a: Box<dyn Adapter>) -> i32 {
        let cnt: i32 = self.adapterMap.len() as i32;
        self.adapterMap[&cnt] = a;
        cnt
    }
}

#[tonic::async_trait]
impl Casbin for CasbinGRPC {
    async fn new_enforcer(
        &self,
        i: Request<NewEnforcerRequest>,
    ) -> Result<Response<NewEnforcerReply>, Status> {
        let a;
        let e: Enforcer;
        if i.into_inner().adapter_handle != -1 {
            a = self.get_adapter(i.into_inner().adapter_handle as i32);
            let response = NewEnforcerReply { handler: 0 };
            return Ok(Response::new(response));
        }

        //if i.into_inner().model_text == "" {
        //    let cfg:
        //}
    }
}
