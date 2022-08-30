use std::collections::HashMap;

use crate::CasbinGRPC;
use casbin::{Adapter, CachedEnforcer};

use futures::lock::Mutex;
use std::sync::Arc;

impl CasbinGRPC {
    pub fn new_server() -> Self {
        Self {
            enforcer_map: HashMap::new(),
            adapter_map: HashMap::new(),
        }
    }

    pub async fn get_enforcer(&self, handle: i32) -> Result<Arc<Mutex<CachedEnforcer>>, &str> {
        let wrapped_cachedenforcer = self.enforcer_map.get(&handle).unwrap().to_owned();
        Ok(wrapped_cachedenforcer)
        // self.enforcer_map.get(&handle).ok_or("No enforcer found")
    }
    pub fn get_adapter(&self, handle: i32) -> Result<&Box<dyn Adapter>, &str> {
        self.adapter_map.get(&handle).ok_or("No adapter found")
    }
    pub fn add_enforcer(&mut self, e: Arc<Mutex<CachedEnforcer>>) -> i32 {
        let cnt: i32 = self.enforcer_map.len() as i32;
        self.enforcer_map.insert(cnt, e);
        cnt
    }
    pub fn add_adapter(&mut self, a: Box<dyn Adapter>) -> i32 {
        let cnt: i32 = self.adapter_map.len() as i32;
        self.adapter_map.insert(cnt, a);
        cnt
    }
}
