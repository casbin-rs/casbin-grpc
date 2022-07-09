use crate::casbin_proto;
use casbin_proto::casbin_server::Casbin;
use tonic::{Request, Response, Status};

use crate::casbin_proto::{
    Array2DReply, ArrayReply, BoolReply, EmptyReply, EmptyRequest, EnforceRequest,
    FilteredPolicyRequest, PolicyRequest, SimpleGetRequest,
};
use crate::server::adapter;
use crate::CasbinGRPC;
use casbin::DefaultModel;
use casbin::MgmtApi;
use casbin::{Adapter, CoreApi, Enforcer, RbacApi};

impl CasbinGRPC {
    pub fn convert_permission(&self, user: String, permissions: Vec<String>) -> Vec<String> {
        let mut params = vec![user];
        for perm in permissions.into_iter() {
            params.push(perm);
        }
        params
    }
}

#[tonic::async_trait]
impl Casbin for CasbinGRPC {
    // RBAC functions here

    // Enforcer functions here
    async fn new_enforcer(
        &self,
        mut request: Request<casbin_proto::NewEnforcerRequest>,
    ) -> Result<Response<casbin_proto::NewEnforcerReply>, Status> {
        let mut a: Option<Box<dyn Adapter>> = None;
        let e: Enforcer;
        if request.get_mut().adapter_handle != -1 {
            a = match self.get_adapter(request.into_inner().adapter_handle) {
                Ok(&v) => Some(v),
                Err(_) => return Ok(Response::new(casbin_proto::NewEnforcerReply { handler: 0 })),
            };
        }
        if request.get_mut().model_text == String::from("") {
            let cfg = adapter::load_configuration("config/connection_config.json").await?;
            let data = match std::fs::read_to_string(cfg.enforcer.as_str()) {
                Ok(v) => v,
                Err(_) => return Ok(Response::new(casbin_proto::NewEnforcerReply { handler: 0 })),
            };
        }

        let m = match DefaultModel::from_str(request.get_mut().model_text.as_str()).await {
            Ok(v) => v,
            Err(_) => return Ok(Response::new(casbin_proto::NewEnforcerReply { handler: 0 })),
        };
        if a.is_none() {
            e = match casbin::Enforcer::new(m, ()).await {
                Ok(v) => v,
                Err(_) => return Ok(Response::new(casbin_proto::NewEnforcerReply { handler: 0 })),
            };
        } else {
            e = match casbin::Enforcer::new(m, *a.unwrap()).await {
                Ok(v) => v,
                Err(_) => return Ok(Response::new(casbin_proto::NewEnforcerReply { handler: 0 })),
            };
        }
        let h = self.add_enforcer(e);
        Ok(Response::new(casbin_proto::NewEnforcerReply { handler: h }))
    }

    async fn new_adapter(
        &self,
        mut request: Request<casbin_proto::NewAdapterRequest>,
    ) -> Result<Response<casbin_proto::NewAdapterReply>, Status> {
        let a = adapter::new_adapter(&mut request)
            .await
            .expect("adapter could not be found");
        let h: i32 = self.add_adapter(a);
        let response = casbin_proto::NewAdapterReply { handler: h };
        Ok(Response::new(response))
    }

    async fn enforce(
        &self,
        request: Request<EnforceRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        todo!()
    }

    async fn load_policy(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<EmptyReply>, Status> {
        todo!()
    }

    async fn save_policy(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<EmptyReply>, Status> {
        todo!()
    }

    async fn add_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        todo!()
    }

    async fn add_named_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        todo!()
    }

    async fn remove_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        todo!()
    }

    async fn remove_named_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        todo!()
    }

    async fn remove_filtered_policy(
        &self,
        request: Request<FilteredPolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        todo!()
    }

    async fn remove_filtered_named_policy(
        &self,
        request: Request<FilteredPolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        todo!()
    }

    async fn get_policy(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<Array2DReply>, Status> {
        todo!()
    }

    async fn get_named_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<Array2DReply>, Status> {
        todo!()
    }

    async fn get_filtered_policy(
        &self,
        request: Request<FilteredPolicyRequest>,
    ) -> Result<Response<Array2DReply>, Status> {
        todo!()
    }

    async fn get_filtered_named_policy(
        &self,
        request: Request<FilteredPolicyRequest>,
    ) -> Result<Response<Array2DReply>, Status> {
        todo!()
    }

    async fn add_grouping_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        todo!()
    }
    async fn add_named_grouping_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        todo!()
    }

    async fn remove_grouping_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        todo!()
    }

    async fn remove_named_grouping_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        todo!()
    }

    async fn remove_filtered_grouping_policy(
        &self,
        request: Request<FilteredPolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        todo!()
    }

    async fn remove_filtered_named_grouping_policy(
        &self,
        request: Request<FilteredPolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        todo!()
    }

    async fn get_grouping_policy(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<Array2DReply>, Status> {
        todo!()
    }

    async fn get_named_grouping_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<Array2DReply>, Status> {
        todo!()
    }

    async fn get_filtered_grouping_policy(
        &self,
        request: Request<FilteredPolicyRequest>,
    ) -> Result<Response<Array2DReply>, Status> {
        todo!()
    }

    async fn get_filtered_named_grouping_policy(
        &self,
        request: Request<FilteredPolicyRequest>,
    ) -> Result<Response<Array2DReply>, Status> {
        todo!()
    }

    async fn get_all_subjects(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<ArrayReply>, Status> {
        todo!()
    }

    async fn get_all_named_subjects(
        &self,
        request: Request<SimpleGetRequest>,
    ) -> Result<Response<ArrayReply>, Status> {
        todo!()
    }

    async fn get_all_objects(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<ArrayReply>, Status> {
        todo!()
    }

    async fn get_all_named_objects(
        &self,
        request: Request<SimpleGetRequest>,
    ) -> Result<Response<ArrayReply>, Status> {
        todo!()
    }

    async fn get_all_actions(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<ArrayReply>, Status> {
        todo!()
    }

    async fn get_all_named_actions(
        &self,
        request: Request<SimpleGetRequest>,
    ) -> Result<Response<ArrayReply>, Status> {
        todo!()
    }

    async fn get_all_roles(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<ArrayReply>, Status> {
        todo!()
    }

    async fn get_all_named_roles(
        &self,
        request: Request<SimpleGetRequest>,
    ) -> Result<Response<ArrayReply>, Status> {
        todo!()
    }

    async fn has_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        todo!()
    }

    async fn has_named_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        todo!()
    }

    async fn has_grouping_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        todo!()
    }

    async fn has_named_grouping_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        todo!()
    }

    // get_roles_for_user gets the roles that a user has.
    async fn get_roles_for_user(
        &self,
        request: Request<casbin_proto::UserRoleRequest>,
    ) -> Result<Response<casbin_proto::ArrayReply>, Status> {
        let e = match self.get_enforcer_mut(request.into_inner().enforcer_handler as i32) {
            Ok(v) => v,
            Err(e) => return Err(Status::new(tonic::Code::NotFound, "Enforcer not found.")),
        };
        let mut roles = vec![];
        if let Some(outer_model) = e.get_mut_model().get_mut_model().get_mut("g") {
            if let Some(inner_model) = outer_model.get_mut("g") {
                // &mut Assertion
                // mut Assertion
                roles = inner_model
                    .rm
                    .write()
                    .get_roles(&*request.into_inner().user, None);
            }
        }
        // let roles_for_user = e.get_model().
        let response = casbin_proto::ArrayReply { array: roles };
        Ok(Response::new(response))
    }

    // get_implicit_roles_for_user gets all permissions(including children) for a user or role.
    async fn get_implicit_roles_for_user(
        &self,
        request: Request<casbin_proto::UserRoleRequest>,
    ) -> Result<Response<casbin_proto::ArrayReply>, Status> {
        let e = match self.get_enforcer_mut(request.into_inner().enforcer_handler as i32) {
            Ok(v) => v,
            Err(e) => return Err(Status::new(tonic::Code::NotFound, "Enforcer not found.")),
        };
        // let implicit_roles_for_user = e.expect("permission not found.");
        let response = casbin_proto::ArrayReply { array: [].to_vec() };
        Ok(Response::new(response))
    }

    // get_users_for_role gets the users that have a role.
    async fn get_users_for_role(
        &self,
        request: Request<casbin_proto::UserRoleRequest>,
    ) -> Result<Response<casbin_proto::ArrayReply>, Status> {
        let enf = match self.get_enforcer_mut(request.into_inner().enforcer_handler as i32) {
            Ok(v) => v,
            Err(e) => return Err(Status::new(tonic::Code::NotFound, "Enforcer not found.")),
        };
        let mut res = vec![];
        if let Some(t1) = enf.get_model().get_model().get("g") {
            if let Some(t2) = t1.get("g") {
                res = t2.rm.read().get_users(&*request.into_inner().user, None);
            }
        }
        let response = casbin_proto::ArrayReply { array: res };
        Ok(Response::new(response))
    }

    //  has_role_for_user determines whether a user has a role.
    async fn has_role_for_user(
        &self,
        request: Request<casbin_proto::UserRoleRequest>,
    ) -> Result<Response<casbin_proto::BoolReply>, Status> {
        let e = match self.get_enforcer_mut(request.into_inner().enforcer_handler as i32) {
            Ok(v) => v,
            Err(er) => return Err(Status::new(tonic::Code::NotFound, "Enforcer not found.")),
        };
        let roles = e.get_roles_for_user(request.into_inner().user.as_str(), None);
        for role in roles.into_iter() {
            if role == request.into_inner().role {
                return Ok(Response::new(casbin_proto::BoolReply { res: true }));
            }
        }
        return Ok(Response::new(casbin_proto::BoolReply { res: false }));
    }

    // add_role_for_user adds a role for a user.
    // Returns false if the user already has the role (aka not affected).
    async fn add_role_for_user(
        &self,
        request: Request<casbin_proto::UserRoleRequest>,
    ) -> Result<Response<casbin_proto::BoolReply>, Status> {
        let e = match self.get_enforcer_mut(request.into_inner().enforcer_handler as i32) {
            Ok(v) => v,
            Err(er) => return Err(Status::new(tonic::Code::NotFound, "Enforcer not found.")),
        };
        let mut user_vec = Vec::new();
        user_vec.push(request.into_inner().user);
        let rule_added = e
            .add_grouping_policy(user_vec)
            .await
            .expect("permission not found.");
        Ok(Response::new(casbin_proto::BoolReply { res: rule_added }))
    }

    // delete_role_for_user deletes a role for a user.
    // Returns false if the user does not have the role (aka not affected).
    async fn delete_role_for_user(
        &self,
        request: Request<casbin_proto::UserRoleRequest>,
    ) -> Result<Response<casbin_proto::BoolReply>, Status> {
        let e = match self.get_enforcer_mut(request.into_inner().enforcer_handler as i32) {
            Ok(v) => v,
            Err(er) => return Err(Status::new(tonic::Code::NotFound, "Enforcer not found.")),
        };
        let mut user_vec = Vec::new();
        user_vec.push(request.into_inner().user);
        let rule_removed = e
            .remove_grouping_policy(user_vec)
            .await
            .expect("permission not found.");
        Ok(Response::new(casbin_proto::BoolReply { res: rule_removed }))
    }

    // delete_roles_for_user deletes all roles for a user.
    // returns false if the user does not have any roles (aka not affected).
    async fn delete_roles_for_user(
        &self,
        request: Request<casbin_proto::UserRoleRequest>,
    ) -> Result<Response<casbin_proto::BoolReply>, Status> {
        let e = match self.get_enforcer_mut(request.into_inner().enforcer_handler as i32) {
            Ok(v) => v,
            Err(er) => return Err(Status::new(tonic::Code::NotFound, "Enforcer not found.")),
        };
        let mut user_vec = Vec::new();
        user_vec.push(request.into_inner().user);
        let rule_removed = e
            .remove_filtered_grouping_policy(0, user_vec)
            .await
            .expect("permission not found.");
        Ok(Response::new(casbin_proto::BoolReply { res: rule_removed }))
    }

    // delete_user deletes a user.
    // Returns false if the user does not exist (aka not affected).
    async fn delete_user(
        &self,
        request: Request<casbin_proto::UserRoleRequest>,
    ) -> Result<Response<casbin_proto::BoolReply>, Status> {
        let e = match self.get_enforcer_mut(request.into_inner().enforcer_handler as i32) {
            Ok(v) => v,
            Err(er) => return Err(Status::new(tonic::Code::NotFound, "Enforcer not found.")),
        };
        let mut user_vec = Vec::new();
        user_vec.push(request.into_inner().user);
        let rule_removed = e
            .remove_filtered_grouping_policy(0, user_vec)
            .await
            .expect("permission not found.");
        Ok(Response::new(casbin_proto::BoolReply { res: rule_removed }))
    }

    // delete_role deletes a role
    async fn delete_role(
        &self,
        request: Request<casbin_proto::UserRoleRequest>,
    ) -> Result<Response<casbin_proto::EmptyReply>, Status> {
        let e = match self.get_enforcer_mut(request.into_inner().enforcer_handler as i32) {
            Ok(v) => v,
            Err(er) => return Err(Status::new(tonic::Code::NotFound, "Enforcer not found.")),
        };
        let _ = e
            .delete_role(&request.into_inner().role)
            .await
            .expect("role not found");
        Ok(Response::new(casbin_proto::EmptyReply {}))
    }

    // get_permissions_for_user gets permissions for a user or role.
    async fn get_permissions_for_user(
        &self,
        request: Request<casbin_proto::PermissionRequest>,
    ) -> Result<Response<casbin_proto::Array2DReply>, Status> {
        let e = match self.get_enforcer_mut(request.into_inner().enforcer_handler as i32) {
            Ok(v) => v,
            Err(er) => return Err(Status::new(tonic::Code::NotFound, "Enforcer not found.")),
        };
        Ok(Response::new(self.wrap_plain_policy(
            e.get_filtered_policy(0, vec![request.into_inner().user]),
        )))
    }

    // get_implicit_permissions_for_user gets all permissions(including children) for a user or role.
    async fn get_implicit_permissions_for_user(
        &self,
        request: Request<casbin_proto::PermissionRequest>,
    ) -> Result<Response<casbin_proto::Array2DReply>, Status> {
        let e = match self.get_enforcer_mut(request.into_inner().enforcer_handler as i32) {
            Ok(v) => v,
            Err(er) => return Err(Status::new(tonic::Code::NotFound, "Enforcer not found.")),
        };
        let resp = e.get_implicit_permissions_for_user(request.into_inner().user.as_str(), None);
        Ok(Response::new(self.wrap_plain_policy(resp)))
    }

    // delete_permission deletes a permission.
    // Returns false if the permission does not exist (aka not affected).
    async fn delete_permission(
        &self,
        request: Request<casbin_proto::PermissionRequest>,
    ) -> Result<Response<casbin_proto::BoolReply>, Status> {
        let e = match self.get_enforcer_mut(request.into_inner().enforcer_handler as i32) {
            Ok(v) => v,
            Err(er) => return Err(Status::new(tonic::Code::NotFound, "Enforcer not found.")),
        };
        let rule_removed = e
            .remove_filtered_policy(1, request.into_inner().permissions)
            .await
            .expect("permissions not found.");
        Ok(Response::new(casbin_proto::BoolReply { res: rule_removed }))
    }

    // add_permission_for user adds a permission for a user or role.
    // Returns false if the user or role already has the permission (aka not affected).
    async fn add_permission_for_user(
        &self,
        request: Request<casbin_proto::PermissionRequest>,
    ) -> Result<Response<casbin_proto::BoolReply>, Status> {
        let e = match self.get_enforcer_mut(request.into_inner().enforcer_handler as i32) {
            Ok(v) => v,
            Err(er) => return Err(Status::new(tonic::Code::NotFound, "Enforcer not found.")),
        };
        let rule_added = e
            .add_policy(request.into_inner().permissions)
            .await
            .expect("permissions not found.");
        Ok(Response::new(casbin_proto::BoolReply { res: rule_added }))
    }

    // delete_permission_for_user deletes a permission for a user or role.
    // Returns false if the user or role does not have the permission (aka not affected).
    async fn delete_permission_for_user(
        &self,
        request: Request<casbin_proto::PermissionRequest>,
    ) -> Result<Response<casbin_proto::BoolReply>, Status> {
        let e = match self.get_enforcer_mut(request.into_inner().enforcer_handler as i32) {
            Ok(v) => v,
            Err(er) => return Err(Status::new(tonic::Code::NotFound, "Enforcer not found.")),
        };
        let rule_removed = e
            .remove_policy(request.into_inner().permissions)
            .await
            .expect("permissions not found.");
        Ok(Response::new(casbin_proto::BoolReply { res: rule_removed }))
    }

    // delete_permissions_for_user deletes permissions for a user or role.
    // Returns false if the user or role does not have any permissions (aka not affected).
    async fn delete_permissions_for_user(
        &self,
        request: Request<casbin_proto::PermissionRequest>,
    ) -> Result<Response<casbin_proto::BoolReply>, Status> {
        let e = match self.get_enforcer_mut(request.into_inner().enforcer_handler as i32) {
            Ok(v) => v,
            Err(er) => return Err(Status::new(tonic::Code::NotFound, "Enforcer not found.")),
        };

        let mut user_vec = Vec::new();
        user_vec.push(request.into_inner().user);
        let rule_removed = e
            .remove_filtered_policy(0, user_vec)
            .await
            .expect("permissions not found.");
        Ok(Response::new(casbin_proto::BoolReply { res: rule_removed }))
    }

    // has_permission_for_user gets determines whether a user has a permission.
    async fn has_permission_for_user(
        &self,
        request: Request<casbin_proto::PermissionRequest>,
    ) -> Result<Response<casbin_proto::BoolReply>, Status> {
        let e = match self.get_enforcer_mut(request.into_inner().enforcer_handler as i32) {
            Ok(v) => v,
            Err(er) => return Err(Status::new(tonic::Code::NotFound, "Enforcer not found.")),
        };
        Ok(Response::new(casbin_proto::BoolReply {
            res: e.has_policy(
                self.convert_permission(
                    request.into_inner().user,
                    request.into_inner().permissions,
                ),
            ),
        }))
    }

    // Management API functions here

    //async fn wrap_plain_policy<'a, Matrix: AsRef[Row]>(
    //    &self,
    //    )
    //// get_all_subjects gets the list of subjects that show up in the current policy.
    //async fn get_all_subjects(
    //    &self,
    //    i: Request<casbin_proto::EmptyRequest>,
    //) -> Result<Response<casbin_proto::ArrayReply>, Status> {
    //    self.get_all_named_subjects()
    //}
}
