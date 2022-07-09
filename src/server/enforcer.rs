use std::collections::HashMap;

use crate::CasbinGRPC;
use casbin::{Adapter, Enforcer};

impl CasbinGRPC {
    pub fn new_server() -> Self {
        Self {
            enforcer_map: HashMap::new(),
            adapter_map: HashMap::new(),
        }
    }
    pub fn get_enforcer(&self, handle: i32) -> Result<&Enforcer, &str> {
        self.enforcer_map.get(&handle).ok_or("No enforcer found")
    }
    pub fn get_enforcer_mut(&mut self, handle: i32) -> Result<&mut Enforcer, &str> {
        self.enforcer_map
            .get_mut(&handle)
            .ok_or("No enforcer found")
    }
    pub fn get_adapter(&self, handle: i32) -> Result<&Box<dyn Adapter>, &str> {
        self.adapter_map.get(&handle).ok_or("No adapter found")
    }
    pub fn add_enforcer(&mut self, e: Enforcer) -> i32 {
        let cnt: i32 = self.enforcer_map.len() as i32;
        self.enforcer_map[&cnt] = e;
        cnt
    }
    pub fn add_adapter(&mut self, a: Box<dyn Adapter>) -> i32 {
        let cnt: i32 = self.adapter_map.len() as i32;
        self.adapter_map[&cnt] = a;
        cnt
    }
}
