use crate::casbin_proto;
use casbin_proto::casbin_server::{Casbin, CasbinServer};
use casbin_proto::{NewEnforcerReply, NewEnforcerRequest};
use std::collections::HashMap;
use tonic::{Request, Response, Status};

use crate::CasbinGRPC;
use casbin::{Adapter, Enforcer};

use std::io::Error as IoError;
impl CasbinGRPC {
    pub fn new_server() -> Self {
        Self {
            enforcerMap: HashMap::new(),
            adapterMap: HashMap::new(),
        }
    }
    pub fn get_enforcer(&self, handle: i32) -> Result<&Enforcer, IoError> {
        let e = match self.enfocerMap.get(&handle) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        return Ok(e);
    }
    pub fn get_adapter(&self, handle: i32) -> Result<&Box<dyn Adapter>, IoError> {
        let a = match self.adapterMap.get(&handle) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        return Ok(a);
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
        let a: Box<dyn Adapter>;
        let e: Enforcer;
        if i.into_inner().adapter_handle != -1 {
            a = match self.get_adapter(i.into_inner().adapter_handle) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
        }

        if i.into_inner().model_text == String::from("") {}
    }
}
