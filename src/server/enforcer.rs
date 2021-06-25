use casbin_proto::{NewEnforcerReply, NewEnforcerRequest};
use std::collections::HashMap;
use tonic::{Request, Response, Status};

use crate::casbin_proto;

use casbin::{Adapter, Enforcer};
use std::error::Error;

pub struct Server {
    enforcerMap: HashMap<i32, Enforcer>,
    adapterMap: HashMap<i32, Box<dyn Adapter>>,
}

impl Server {
    pub fn NewServer() -> Self {
        Self {
            enforcerMap: HashMap::new(),
            adapterMap: HashMap::new(),
        }
    }
}

impl Server {
    pub fn getEnforcer(&self, handle: i32) -> Option<&Enforcer> {
        self.enforcerMap.get(&handle)
    }

    pub fn getAdapter(&self, handle: i32) -> Option<&Box<dyn Adapter>> {
        self.adapterMap.get(&handle)
    }
    pub fn addEnforcer(&self, e: Enforcer) -> i32 {
        let cnt: usize = self.enforcerMap.len();
        self.enforcerMap[&cnt] = e;
        cnt
    }
    pub fn addAdapter(&self, a: Box<dyn Adapter>) -> i32 {
        let cnt: i32 = self.adapterMap.len();
        self.adapterMap[&cnt] = a;
        cnt
    }
    pub fn NewEnforcer(
        &self,
        i: Request<NewEnforcerRequest>,
    ) -> Result<Response<NewEnforcerReply>, Status> {
        let a: Box<dyn Adapter>;
        let e: Enforcer;
        if i.into_inner().adapter_handle != -1 {
            a = self.getAdapter(i.into_inner().adapter_handle as i32);
            let response = NewEnforcerReply { handler: 0 };
            Ok(Response::new(response))
        }
    }
}
