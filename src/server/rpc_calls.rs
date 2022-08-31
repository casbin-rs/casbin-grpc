use std::sync::Arc;

use crate::casbin_proto;
use crate::casbin_proto::casbin_server::Casbin;
use crate::casbin_proto::{
    Array2DReply, ArrayReply, BoolReply, EmptyReply, EmptyRequest, FilteredPolicyRequest,
    PolicyRequest, SimpleGetRequest,
};
use futures::lock::Mutex;
use tonic::{Request, Response, Status};

use crate::server::adapter;
use crate::CasbinGRPC;
use casbin::MgmtApi;
use casbin::{Adapter, CoreApi, RbacApi};
use casbin::{CachedEnforcer, DefaultModel};

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

    // get_roles_for_user gets the roles that a user has.
    async fn get_roles_for_user(
        &self,
        request: Request<casbin_proto::UserRoleRequest>,
    ) -> Result<Response<casbin_proto::ArrayReply>, Status> {
        let get_inner = request.into_inner();

        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let e = wrap_enforcer.lock().await;
        let mut roles = vec![];
        if let Some(outer_model) = e.get_model().get_model().to_owned().get_mut("g") {
            if let Some(inner_model) = outer_model.get_mut("g") {
                // &mut Assertion
                // mut Assertion
                roles = inner_model.rm.write().get_roles(&get_inner.user, None);
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
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let e = wrap_enforcer.lock().await;
        // let implicit_roles_for_user = e.expect("permission not found.");
        let response = casbin_proto::ArrayReply { array: [].to_vec() };
        Ok(Response::new(response))
    }

    // get_users_for_role gets the users that have a role.
    async fn get_users_for_role(
        &self,
        request: Request<casbin_proto::UserRoleRequest>,
    ) -> Result<Response<casbin_proto::ArrayReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let e = wrap_enforcer.lock().await;
        let mut res = vec![];
        if let Some(t1) = e.get_model().get_model().get("g") {
            if let Some(t2) = t1.get("g") {
                res = t2.rm.read().get_users(&get_inner.user, None);
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
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let mut e = wrap_enforcer.lock().await;

        let roles = e.get_roles_for_user(get_inner.user.as_str(), None);
        for role in roles.into_iter() {
            if role == get_inner.role {
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
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let mut e = wrap_enforcer.lock().await;
        let mut user_vec = Vec::new();
        user_vec.push(get_inner.user);
        let rule_added = e
            .add_grouping_policy(user_vec)
            .await
            .expect("permission not found.");
        Ok(Response::new(casbin_proto::BoolReply { res: rule_added }))
    }

    // delete_role_for_user deletes a role for a user.
    // Returns false if the user does not have the role (aka not affected).

    // Use middleware
    async fn delete_role_for_user(
        &self,
        request: Request<casbin_proto::UserRoleRequest>,
    ) -> Result<Response<casbin_proto::BoolReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let mut e = wrap_enforcer.lock().await;
        let mut user_vec = Vec::new();
        user_vec.push(get_inner.user);
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
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let mut e = wrap_enforcer.lock().await;
        let mut user_vec = Vec::new();
        user_vec.push(get_inner.user);
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
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let mut e = wrap_enforcer.lock().await;
        let mut user_vec = Vec::new();
        user_vec.push(get_inner.user);
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
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let mut e = wrap_enforcer.lock().await;
        let _ = e
            .delete_role(&get_inner.role)
            .await
            .expect("role not found");
        Ok(Response::new(casbin_proto::EmptyReply {}))
    }

    // delete_permission deletes a permission.
    // Returns false if the permission does not exist (aka not affected).
    async fn delete_permission(
        &self,
        request: Request<casbin_proto::PermissionRequest>,
    ) -> Result<Response<casbin_proto::BoolReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let mut e = wrap_enforcer.lock().await;
        let rule_removed = e
            .remove_filtered_policy(1, get_inner.permissions)
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
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let mut e = wrap_enforcer.lock().await;
        let rule_added = e
            .add_policy(get_inner.permissions)
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
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let mut e = wrap_enforcer.lock().await;
        let rule_removed = e
            .remove_policy(get_inner.permissions)
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
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let mut e = wrap_enforcer.lock().await;

        let mut user_vec = Vec::new();
        user_vec.push(get_inner.user);
        let rule_removed = e
            .remove_filtered_policy(0, user_vec)
            .await
            .expect("permissions not found.");
        Ok(Response::new(casbin_proto::BoolReply { res: rule_removed }))
    }

    // get_permissions_for_user gets permissions for a user or role.
    async fn get_permissions_for_user(
        &self,
        request: Request<casbin_proto::PermissionRequest>,
    ) -> Result<Response<casbin_proto::Array2DReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let e = wrap_enforcer.lock().await;
        Ok(Response::new(self.wrap_plain_policy(
            e.get_filtered_policy(0, vec![get_inner.user]),
        )))
    }

    // get_implicit_permissions_for_user gets all permissions(including children) for a user or role.
    async fn get_implicit_permissions_for_user(
        &self,
        request: Request<casbin_proto::PermissionRequest>,
    ) -> Result<Response<casbin_proto::Array2DReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let mut e = wrap_enforcer.lock().await;
        let resp = e.get_implicit_permissions_for_user(get_inner.user.as_str(), None);
        Ok(Response::new(self.wrap_plain_policy(resp)))
    }

    // has_permission_for_user gets determines whether a user has a permission.
    async fn has_permission_for_user(
        &self,
        request: Request<casbin_proto::PermissionRequest>,
    ) -> Result<Response<casbin_proto::BoolReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let e = wrap_enforcer.lock().await;
        Ok(Response::new(casbin_proto::BoolReply {
            res: e.has_policy(self.convert_permission(get_inner.user, get_inner.permissions)),
        }))
    }
    // Enforcer functions here
    async fn new_enforcer(
        &self,
        i: Request<casbin_proto::NewEnforcerRequest>,
    ) -> Result<Response<casbin_proto::NewEnforcerReply>, Status> {
        todo!()
    }

    async fn new_adapter(
        &self,
        mut i: Request<casbin_proto::NewAdapterRequest>,
    ) -> Result<Response<casbin_proto::NewAdapterReply>, Status> {
        todo!()
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

    async fn enforce(
        &self,
        request: Request<casbin_proto::EnforceRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        todo!()

        // res, err := e.EnforceWithMatcher(m, params...)
    }

    async fn load_policy(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<EmptyReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self.get_enforcer(get_inner.handler as i32).await.unwrap();
        let e = wrap_enforcer.lock().await;
        Ok(Response::new(casbin_proto::EmptyReply {}))
    }

    async fn save_policy(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<EmptyReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self.get_enforcer(get_inner.handler as i32).await.unwrap();
        let e = wrap_enforcer.lock().await;

        Ok(Response::new(casbin_proto::EmptyReply {}))
    }

    async fn add_policy(
        &self,
        mut request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        let mut get_inner = request.get_mut();
        get_inner.p_type = String::from("p");

        Ok(self.add_named_policy(request).await.unwrap())
    }

    async fn add_named_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let mut e = wrap_enforcer.lock().await;
        let rule_added = e
            .add_named_policy(&get_inner.p_type, get_inner.params)
            .await
            .unwrap();

        Ok(Response::new(casbin_proto::BoolReply { res: rule_added }))
    }

    async fn remove_policy(
        &self,
        mut request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        let mut get_inner = request.get_mut();
        get_inner.p_type = String::from("p");

        Ok(self.remove_named_policy(request).await.unwrap())
    }

    async fn remove_named_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let mut e = wrap_enforcer.lock().await;
        let rule_removed = e
            .remove_named_policy(&get_inner.p_type, get_inner.params)
            .await
            .unwrap();
        Ok(Response::new(casbin_proto::BoolReply { res: rule_removed }))
    }

    async fn remove_filtered_policy(
        &self,
        mut request: Request<FilteredPolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        let mut get_inner = request.get_mut();
        get_inner.p_type = String::from("p");

        Ok(self.remove_filtered_named_policy(request).await.unwrap())
    }

    async fn remove_filtered_named_policy(
        &self,
        request: Request<FilteredPolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let mut e = wrap_enforcer.lock().await;

        let rule_removed_filtered = e
            .remove_filtered_named_policy(
                &get_inner.p_type,
                get_inner.field_index as usize,
                get_inner.field_values,
            )
            .await
            .unwrap();

        Ok(Response::new(casbin_proto::BoolReply {
            res: rule_removed_filtered,
        }))
    }

    async fn get_policy(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<Array2DReply>, Status> {
        Ok(self
            .get_named_policy(Request::new(casbin_proto::PolicyRequest {
                enforcer_handler: request.into_inner().handler,
                p_type: String::from("p"),
                params: vec![String::from("")],
            }))
            .await
            .unwrap())
    }

    async fn get_named_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<Array2DReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let e = wrap_enforcer.lock().await;

        Ok(Response::new(self.wrap_plain_policy(
            e.get_model().get_policy("p", &*get_inner.p_type),
        )))
    }

    async fn get_filtered_policy(
        &self,
        mut request: Request<FilteredPolicyRequest>,
    ) -> Result<Response<Array2DReply>, Status> {
        let mut get_inner = request.get_mut();
        get_inner.p_type = String::from("p");

        Ok(self.get_filtered_named_policy(request).await.unwrap())
    }

    async fn get_filtered_named_policy(
        &self,
        request: Request<FilteredPolicyRequest>,
    ) -> Result<Response<Array2DReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let e = wrap_enforcer.lock().await;

        Ok(Response::new(self.wrap_plain_policy(
            e.get_model().get_filtered_policy(
                "p",
                &*get_inner.p_type,
                get_inner.field_index as usize,
                get_inner.field_values,
            ),
        )))
    }

    async fn add_grouping_policy(
        &self,
        mut request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        let mut get_inner = request.get_mut();
        get_inner.p_type = String::from("g");

        Ok(self.add_named_grouping_policy(request).await.unwrap())
    }
    async fn add_named_grouping_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let mut e = wrap_enforcer.lock().await;

        let rule_added = e
            .add_named_grouping_policy(&get_inner.p_type, get_inner.params)
            .await
            .unwrap();
        Ok(Response::new(casbin_proto::BoolReply { res: rule_added }))
    }

    async fn remove_grouping_policy(
        &self,
        mut request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        let mut get_inner = request.get_mut();
        get_inner.p_type = String::from("g");

        Ok(self.remove_named_grouping_policy(request).await.unwrap())
    }

    async fn remove_named_grouping_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let mut e = wrap_enforcer.lock().await;

        let rule_removed = e
            .remove_named_grouping_policy(&get_inner.p_type, get_inner.params)
            .await
            .unwrap();
        Ok(Response::new(casbin_proto::BoolReply { res: rule_removed }))
    }

    async fn remove_filtered_grouping_policy(
        &self,
        mut request: Request<FilteredPolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        let mut get_inner = request.get_mut();
        get_inner.p_type = String::from("g");

        Ok(self
            .remove_filtered_named_grouping_policy(request)
            .await
            .unwrap())
    }

    async fn remove_filtered_named_grouping_policy(
        &self,
        request: Request<FilteredPolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let mut e = wrap_enforcer.lock().await;

        let rule_filtered_removed = e
            .remove_filtered_named_grouping_policy(
                &get_inner.p_type,
                get_inner.field_index as usize,
                get_inner.field_values,
            )
            .await
            .unwrap();
        Ok(Response::new(casbin_proto::BoolReply {
            res: rule_filtered_removed,
        }))
    }

    async fn get_grouping_policy(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<Array2DReply>, Status> {
        let get_inner = request.into_inner();
        Ok(self
            .get_named_grouping_policy(Request::new(casbin_proto::PolicyRequest {
                enforcer_handler: get_inner.handler,
                p_type: String::from("g"),
                params: vec![String::from("")],
            }))
            .await
            .unwrap())
    }

    async fn get_named_grouping_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<Array2DReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let e = wrap_enforcer.lock().await;

        Ok(Response::new(self.wrap_plain_policy(
            e.get_model().get_policy("p", &get_inner.p_type),
        )))
    }

    async fn get_filtered_grouping_policy(
        &self,
        mut request: Request<FilteredPolicyRequest>,
    ) -> Result<Response<Array2DReply>, Status> {
        let mut get_inner = request.get_mut();
        get_inner.p_type = String::from("g");

        Ok(self
            .get_filtered_named_grouping_policy(request)
            .await
            .unwrap())
    }

    async fn get_filtered_named_grouping_policy(
        &self,
        request: Request<FilteredPolicyRequest>,
    ) -> Result<Response<Array2DReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let e = wrap_enforcer.lock().await;

        Ok(Response::new(self.wrap_plain_policy(
            e.get_model().get_filtered_policy(
                "g",
                &get_inner.p_type,
                get_inner.field_index as usize,
                get_inner.field_values,
            ),
        )))
    }

    async fn get_all_subjects(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<ArrayReply>, Status> {
        Ok(self
            .get_all_named_subjects(Request::new(casbin_proto::SimpleGetRequest {
                enforcer_handler: request.into_inner().handler,
                p_type: String::from("p"),
            }))
            .await
            .unwrap())
    }

    async fn get_all_named_subjects(
        &self,
        mut request: Request<SimpleGetRequest>,
    ) -> Result<Response<ArrayReply>, Status> {
        let wrap_enforcer = self
            .get_enforcer(request.get_mut().enforcer_handler as i32)
            .await
            .unwrap();
        let e = wrap_enforcer.lock().await;

        Ok(Response::new(casbin_proto::ArrayReply {
            array: e.get_model().get_values_for_field_in_policy(
                "p",
                &request.into_inner().p_type,
                0,
            ),
        }))
    }

    async fn get_all_objects(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<ArrayReply>, Status> {
        Ok(self
            .get_all_named_objects(Request::new(casbin_proto::SimpleGetRequest {
                enforcer_handler: request.into_inner().handler,
                p_type: String::from("p"),
            }))
            .await
            .unwrap())
    }

    async fn get_all_named_objects(
        &self,
        mut request: Request<SimpleGetRequest>,
    ) -> Result<Response<ArrayReply>, Status> {
        let wrap_enforcer = self
            .get_enforcer(request.get_mut().enforcer_handler as i32)
            .await
            .unwrap();
        let e = wrap_enforcer.lock().await;

        Ok(Response::new(casbin_proto::ArrayReply {
            array: e.get_model().get_values_for_field_in_policy(
                "p",
                &request.into_inner().p_type,
                1,
            ),
        }))
    }

    async fn get_all_actions(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<ArrayReply>, Status> {
        Ok(self
            .get_all_named_objects(Request::new(casbin_proto::SimpleGetRequest {
                enforcer_handler: request.into_inner().handler,
                p_type: String::from("p"),
            }))
            .await
            .unwrap())
    }

    async fn get_all_named_actions(
        &self,
        mut request: Request<SimpleGetRequest>,
    ) -> Result<Response<ArrayReply>, Status> {
        let wrap_enforcer = self
            .get_enforcer(request.get_mut().enforcer_handler as i32)
            .await
            .unwrap();
        let e = wrap_enforcer.lock().await;

        Ok(Response::new(casbin_proto::ArrayReply {
            array: e.get_model().get_values_for_field_in_policy(
                "p",
                &request.into_inner().p_type,
                2,
            ),
        }))
    }

    async fn get_all_roles(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<ArrayReply>, Status> {
        Ok(self
            .get_all_named_objects(Request::new(casbin_proto::SimpleGetRequest {
                enforcer_handler: request.into_inner().handler,
                p_type: String::from("g"),
            }))
            .await
            .unwrap())
    }

    async fn get_all_named_roles(
        &self,
        mut request: Request<SimpleGetRequest>,
    ) -> Result<Response<ArrayReply>, Status> {
        let wrap_enforcer = self
            .get_enforcer(request.get_mut().enforcer_handler as i32)
            .await
            .unwrap();
        let e = wrap_enforcer.lock().await;

        Ok(Response::new(casbin_proto::ArrayReply {
            array: e.get_model().get_values_for_field_in_policy(
                "g",
                &request.into_inner().p_type,
                1,
            ),
        }))
    }

    async fn has_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        Ok(self.has_named_policy(request).await.unwrap())
    }

    async fn has_named_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let e = wrap_enforcer.lock().await;

        Ok(Response::new(casbin_proto::BoolReply {
            res: e
                .get_model()
                .has_policy("p", &get_inner.p_type, get_inner.params),
        }))
    }

    async fn has_grouping_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        let mut get_inner = request.into_inner();
        get_inner.p_type = String::from("g");
        let req_new = Request::new(get_inner);
        Ok(self.has_named_grouping_policy(req_new).await.unwrap())
    }

    async fn has_named_grouping_policy(
        &self,
        request: Request<PolicyRequest>,
    ) -> Result<Response<BoolReply>, Status> {
        let get_inner = request.into_inner();
        let wrap_enforcer = self
            .get_enforcer(get_inner.enforcer_handler as i32)
            .await
            .unwrap();
        let e = wrap_enforcer.lock().await;

        Ok(Response::new(casbin_proto::BoolReply {
            res: e
                .get_model()
                .has_policy("g", &get_inner.p_type, get_inner.params),
        }))
    }
}
