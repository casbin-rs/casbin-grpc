use std::collections::HashMap;

use crate::{CasbinGRPC};
use crate::server::abac;
use casbin::{Adapter, CachedEnforcer};

use std::sync::{Arc};
use futures::lock::Mutex;



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
    pub fn parse_param(
        &self,
        param: String,
        matcher: &mut String,
    ) -> Result<abac::AbacAttrList, String> {
        if param.starts_with("ABAC::") {
            let attr_list = abac::resolve_abac(String::from(param)).expect("");
            for (k, v) in attr_list.name_map.iter() {
                let old: String = format!("{}{}", ".", &k);
                let new: String = format!("{}{}", ".", &v);
                if matcher.contains(&old) {
                    matcher.replace(&old, &new);
                }
            }
            return Ok(attr_list);
        } else {
            return Err(param);
        }
    }

}
