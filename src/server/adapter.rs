use crate::casbin_proto;
use casbin_proto::casbin_server::{Casbin, CasbinServer};
use casbin_proto::{NewEnforcerReply, NewEnforcerRequest};
use std::collections::HashMap;
use tonic::{Request, Response, Status};

use crate::CasbinGRPC;
use std::error::Error;

//let err_driver_name =
