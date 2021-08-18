use crate::casbin_proto;
use crate::CasbinGRPC;
//use casbin::{Adapter, Enforcer};
//use tonic::Response;

use casbin_proto::{array2_d_reply, Array2DReply};

impl CasbinGRPC {
    pub fn wrap_plain_policy(&self, policy: Vec<Vec<String>>) -> Array2DReply {
        if policy.len() == 0 {
            return Array2DReply {
                d2: vec![array2_d_reply::D {
                    d1: vec![String::from("not found.")],
                }],
            };
        } else {
            let mut policy_reply: Array2DReply = Array2DReply { d2: vec![] };
            for e in 1..policy.len() {
                policy_reply.d2.push(array2_d_reply::D {
                    d1: policy[e].to_vec(),
                })
            }
            policy_reply
        }
    }
}
