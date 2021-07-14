use std::collections::HashMap;

use crate::CasbinGRPC;
use casbin::{Adapter, Enforcer};

impl CasbinGRPC {
    pub fn new_server() -> Self {
        Self {
            enforcerMap: HashMap::new(),
            adapterMap: HashMap::new(),
        }
    }
    pub fn get_enforcer(&self, handle: i32) -> Result<&Enforcer, &str> {
        self.enforcerMap.get(&handle).ok_or("No enforcer found")
    }
    pub fn get_adapter(&self, handle: i32) -> Result<&Box<dyn Adapter>, &str> {
        self.adapterMap.get(&handle).ok_or("No adapter found")
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
