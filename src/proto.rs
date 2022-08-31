#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEnforcerRequest {
    #[prost(string, tag = "1")]
    pub model_text: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub adapter_handle: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewEnforcerReply {
    #[prost(int32, tag = "1")]
    pub handler: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAdapterRequest {
    #[prost(string, tag = "1")]
    pub adapter_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub driver_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub connect_string: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub db_specified: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAdapterReply {
    #[prost(int32, tag = "1")]
    pub handler: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnforceRequest {
    #[prost(int32, tag = "1")]
    pub enforcer_handler: i32,
    #[prost(string, repeated, tag = "2")]
    pub params: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoolReply {
    #[prost(bool, tag = "1")]
    pub res: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyRequest {
    #[prost(int32, tag = "1")]
    pub handler: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmptyReply {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyRequest {
    #[prost(int32, tag = "1")]
    pub enforcer_handler: i32,
    #[prost(string, tag = "2")]
    pub p_type: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub params: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleGetRequest {
    #[prost(int32, tag = "1")]
    pub enforcer_handler: i32,
    #[prost(string, tag = "2")]
    pub p_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrayReply {
    #[prost(string, repeated, tag = "1")]
    pub array: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilteredPolicyRequest {
    #[prost(int32, tag = "1")]
    pub enforcer_handler: i32,
    #[prost(string, tag = "2")]
    pub p_type: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub field_index: i32,
    #[prost(string, repeated, tag = "4")]
    pub field_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRoleRequest {
    #[prost(int32, tag = "1")]
    pub enforcer_handler: i32,
    #[prost(string, tag = "2")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub role: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PermissionRequest {
    #[prost(int32, tag = "1")]
    pub enforcer_handler: i32,
    #[prost(string, tag = "2")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Array2DReply {
    #[prost(message, repeated, tag = "1")]
    pub d2: ::prost::alloc::vec::Vec<array2_d_reply::D>,
}
/// Nested message and enum types in `Array2DReply`.
pub mod array2_d_reply {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct D {
        #[prost(string, repeated, tag = "1")]
        pub d1: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Generated client implementations.
pub mod casbin_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// The Casbin service definition.
    #[derive(Debug, Clone)]
    pub struct CasbinClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CasbinClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CasbinClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CasbinClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            CasbinClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn new_enforcer(
            &mut self,
            request: impl tonic::IntoRequest<super::NewEnforcerRequest>,
        ) -> Result<tonic::Response<super::NewEnforcerReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/NewEnforcer");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn new_adapter(
            &mut self,
            request: impl tonic::IntoRequest<super::NewAdapterRequest>,
        ) -> Result<tonic::Response<super::NewAdapterReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/NewAdapter");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn enforce(
            &mut self,
            request: impl tonic::IntoRequest<super::EnforceRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/Enforce");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn load_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::EmptyReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/LoadPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn save_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::EmptyReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/SavePolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/AddPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_named_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/AddNamedPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/RemovePolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_named_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/RemoveNamedPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_filtered_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::FilteredPolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/RemoveFilteredPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_filtered_named_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::FilteredPolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/proto.Casbin/RemoveFilteredNamedPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/GetPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_named_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/GetNamedPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_filtered_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::FilteredPolicyRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/GetFilteredPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_filtered_named_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::FilteredPolicyRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/GetFilteredNamedPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_grouping_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/AddGroupingPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_named_grouping_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/AddNamedGroupingPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_grouping_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/RemoveGroupingPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_named_grouping_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/proto.Casbin/RemoveNamedGroupingPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_filtered_grouping_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::FilteredPolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/proto.Casbin/RemoveFilteredGroupingPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_filtered_named_grouping_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::FilteredPolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Casbin/RemoveFilteredNamedGroupingPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_grouping_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/GetGroupingPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_named_grouping_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/GetNamedGroupingPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_filtered_grouping_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::FilteredPolicyRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/proto.Casbin/GetFilteredGroupingPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_filtered_named_grouping_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::FilteredPolicyRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.Casbin/GetFilteredNamedGroupingPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_all_subjects(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/GetAllSubjects");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_all_named_subjects(
            &mut self,
            request: impl tonic::IntoRequest<super::SimpleGetRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/GetAllNamedSubjects");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_all_objects(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/GetAllObjects");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_all_named_objects(
            &mut self,
            request: impl tonic::IntoRequest<super::SimpleGetRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/GetAllNamedObjects");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_all_actions(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/GetAllActions");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_all_named_actions(
            &mut self,
            request: impl tonic::IntoRequest<super::SimpleGetRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/GetAllNamedActions");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_all_roles(
            &mut self,
            request: impl tonic::IntoRequest<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/GetAllRoles");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_all_named_roles(
            &mut self,
            request: impl tonic::IntoRequest<super::SimpleGetRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/GetAllNamedRoles");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn has_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/HasPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn has_named_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/HasNamedPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn has_grouping_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/HasGroupingPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn has_named_grouping_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/HasNamedGroupingPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_roles_for_user(
            &mut self,
            request: impl tonic::IntoRequest<super::UserRoleRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/GetRolesForUser");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_implicit_roles_for_user(
            &mut self,
            request: impl tonic::IntoRequest<super::UserRoleRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/proto.Casbin/GetImplicitRolesForUser");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_users_for_role(
            &mut self,
            request: impl tonic::IntoRequest<super::UserRoleRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/GetUsersForRole");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn has_role_for_user(
            &mut self,
            request: impl tonic::IntoRequest<super::UserRoleRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/HasRoleForUser");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_role_for_user(
            &mut self,
            request: impl tonic::IntoRequest<super::UserRoleRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/AddRoleForUser");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_role_for_user(
            &mut self,
            request: impl tonic::IntoRequest<super::UserRoleRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/DeleteRoleForUser");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_roles_for_user(
            &mut self,
            request: impl tonic::IntoRequest<super::UserRoleRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/DeleteRolesForUser");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_user(
            &mut self,
            request: impl tonic::IntoRequest<super::UserRoleRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/DeleteUser");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_role(
            &mut self,
            request: impl tonic::IntoRequest<super::UserRoleRequest>,
        ) -> Result<tonic::Response<super::EmptyReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/DeleteRole");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_permissions_for_user(
            &mut self,
            request: impl tonic::IntoRequest<super::PermissionRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/GetPermissionsForUser");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_implicit_permissions_for_user(
            &mut self,
            request: impl tonic::IntoRequest<super::PermissionRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/proto.Casbin/GetImplicitPermissionsForUser");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_permission(
            &mut self,
            request: impl tonic::IntoRequest<super::PermissionRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/DeletePermission");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_permission_for_user(
            &mut self,
            request: impl tonic::IntoRequest<super::PermissionRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/AddPermissionForUser");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_permission_for_user(
            &mut self,
            request: impl tonic::IntoRequest<super::PermissionRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/proto.Casbin/DeletePermissionForUser");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_permissions_for_user(
            &mut self,
            request: impl tonic::IntoRequest<super::PermissionRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/proto.Casbin/DeletePermissionsForUser");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn has_permission_for_user(
            &mut self,
            request: impl tonic::IntoRequest<super::PermissionRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/proto.Casbin/HasPermissionForUser");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod casbin_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with CasbinServer.
    #[async_trait]
    pub trait Casbin: Send + Sync + 'static {
        async fn new_enforcer(
            &self,
            request: tonic::Request<super::NewEnforcerRequest>,
        ) -> Result<tonic::Response<super::NewEnforcerReply>, tonic::Status>;
        async fn new_adapter(
            &self,
            request: tonic::Request<super::NewAdapterRequest>,
        ) -> Result<tonic::Response<super::NewAdapterReply>, tonic::Status>;
        async fn enforce(
            &self,
            request: tonic::Request<super::EnforceRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn load_policy(
            &self,
            request: tonic::Request<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::EmptyReply>, tonic::Status>;
        async fn save_policy(
            &self,
            request: tonic::Request<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::EmptyReply>, tonic::Status>;
        async fn add_policy(
            &self,
            request: tonic::Request<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn add_named_policy(
            &self,
            request: tonic::Request<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn remove_policy(
            &self,
            request: tonic::Request<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn remove_named_policy(
            &self,
            request: tonic::Request<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn remove_filtered_policy(
            &self,
            request: tonic::Request<super::FilteredPolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn remove_filtered_named_policy(
            &self,
            request: tonic::Request<super::FilteredPolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn get_policy(
            &self,
            request: tonic::Request<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status>;
        async fn get_named_policy(
            &self,
            request: tonic::Request<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status>;
        async fn get_filtered_policy(
            &self,
            request: tonic::Request<super::FilteredPolicyRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status>;
        async fn get_filtered_named_policy(
            &self,
            request: tonic::Request<super::FilteredPolicyRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status>;
        async fn add_grouping_policy(
            &self,
            request: tonic::Request<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn add_named_grouping_policy(
            &self,
            request: tonic::Request<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn remove_grouping_policy(
            &self,
            request: tonic::Request<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn remove_named_grouping_policy(
            &self,
            request: tonic::Request<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn remove_filtered_grouping_policy(
            &self,
            request: tonic::Request<super::FilteredPolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn remove_filtered_named_grouping_policy(
            &self,
            request: tonic::Request<super::FilteredPolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn get_grouping_policy(
            &self,
            request: tonic::Request<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status>;
        async fn get_named_grouping_policy(
            &self,
            request: tonic::Request<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status>;
        async fn get_filtered_grouping_policy(
            &self,
            request: tonic::Request<super::FilteredPolicyRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status>;
        async fn get_filtered_named_grouping_policy(
            &self,
            request: tonic::Request<super::FilteredPolicyRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status>;
        async fn get_all_subjects(
            &self,
            request: tonic::Request<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status>;
        async fn get_all_named_subjects(
            &self,
            request: tonic::Request<super::SimpleGetRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status>;
        async fn get_all_objects(
            &self,
            request: tonic::Request<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status>;
        async fn get_all_named_objects(
            &self,
            request: tonic::Request<super::SimpleGetRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status>;
        async fn get_all_actions(
            &self,
            request: tonic::Request<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status>;
        async fn get_all_named_actions(
            &self,
            request: tonic::Request<super::SimpleGetRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status>;
        async fn get_all_roles(
            &self,
            request: tonic::Request<super::EmptyRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status>;
        async fn get_all_named_roles(
            &self,
            request: tonic::Request<super::SimpleGetRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status>;
        async fn has_policy(
            &self,
            request: tonic::Request<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn has_named_policy(
            &self,
            request: tonic::Request<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn has_grouping_policy(
            &self,
            request: tonic::Request<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn has_named_grouping_policy(
            &self,
            request: tonic::Request<super::PolicyRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn get_roles_for_user(
            &self,
            request: tonic::Request<super::UserRoleRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status>;
        async fn get_implicit_roles_for_user(
            &self,
            request: tonic::Request<super::UserRoleRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status>;
        async fn get_users_for_role(
            &self,
            request: tonic::Request<super::UserRoleRequest>,
        ) -> Result<tonic::Response<super::ArrayReply>, tonic::Status>;
        async fn has_role_for_user(
            &self,
            request: tonic::Request<super::UserRoleRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn add_role_for_user(
            &self,
            request: tonic::Request<super::UserRoleRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn delete_role_for_user(
            &self,
            request: tonic::Request<super::UserRoleRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn delete_roles_for_user(
            &self,
            request: tonic::Request<super::UserRoleRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn delete_user(
            &self,
            request: tonic::Request<super::UserRoleRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn delete_role(
            &self,
            request: tonic::Request<super::UserRoleRequest>,
        ) -> Result<tonic::Response<super::EmptyReply>, tonic::Status>;
        async fn get_permissions_for_user(
            &self,
            request: tonic::Request<super::PermissionRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status>;
        async fn get_implicit_permissions_for_user(
            &self,
            request: tonic::Request<super::PermissionRequest>,
        ) -> Result<tonic::Response<super::Array2DReply>, tonic::Status>;
        async fn delete_permission(
            &self,
            request: tonic::Request<super::PermissionRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn add_permission_for_user(
            &self,
            request: tonic::Request<super::PermissionRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn delete_permission_for_user(
            &self,
            request: tonic::Request<super::PermissionRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn delete_permissions_for_user(
            &self,
            request: tonic::Request<super::PermissionRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
        async fn has_permission_for_user(
            &self,
            request: tonic::Request<super::PermissionRequest>,
        ) -> Result<tonic::Response<super::BoolReply>, tonic::Status>;
    }
    /// The Casbin service definition.
    #[derive(Debug)]
    pub struct CasbinServer<T: Casbin> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Casbin> CasbinServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CasbinServer<T>
    where
        T: Casbin,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/proto.Casbin/NewEnforcer" => {
                    #[allow(non_camel_case_types)]
                    struct NewEnforcerSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::NewEnforcerRequest> for NewEnforcerSvc<T> {
                        type Response = super::NewEnforcerReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewEnforcerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_enforcer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewEnforcerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/NewAdapter" => {
                    #[allow(non_camel_case_types)]
                    struct NewAdapterSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::NewAdapterRequest> for NewAdapterSvc<T> {
                        type Response = super::NewAdapterReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewAdapterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_adapter(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewAdapterSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/Enforce" => {
                    #[allow(non_camel_case_types)]
                    struct EnforceSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::EnforceRequest> for EnforceSvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EnforceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).enforce(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EnforceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/LoadPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct LoadPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::EmptyRequest> for LoadPolicySvc<T> {
                        type Response = super::EmptyReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).load_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LoadPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/SavePolicy" => {
                    #[allow(non_camel_case_types)]
                    struct SavePolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::EmptyRequest> for SavePolicySvc<T> {
                        type Response = super::EmptyReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).save_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SavePolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/AddPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct AddPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for AddPolicySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/AddNamedPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct AddNamedPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for AddNamedPolicySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_named_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddNamedPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/RemovePolicy" => {
                    #[allow(non_camel_case_types)]
                    struct RemovePolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for RemovePolicySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemovePolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/RemoveNamedPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveNamedPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for RemoveNamedPolicySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_named_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveNamedPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/RemoveFilteredPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveFilteredPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::FilteredPolicyRequest>
                        for RemoveFilteredPolicySvc<T>
                    {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FilteredPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_filtered_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveFilteredPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/RemoveFilteredNamedPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveFilteredNamedPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::FilteredPolicyRequest>
                        for RemoveFilteredNamedPolicySvc<T>
                    {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FilteredPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).remove_filtered_named_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveFilteredNamedPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::EmptyRequest> for GetPolicySvc<T> {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetNamedPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetNamedPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for GetNamedPolicySvc<T> {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_named_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNamedPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetFilteredPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetFilteredPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::FilteredPolicyRequest>
                        for GetFilteredPolicySvc<T>
                    {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FilteredPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_filtered_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFilteredPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetFilteredNamedPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetFilteredNamedPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::FilteredPolicyRequest>
                        for GetFilteredNamedPolicySvc<T>
                    {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FilteredPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_filtered_named_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFilteredNamedPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/AddGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct AddGroupingPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for AddGroupingPolicySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_grouping_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddGroupingPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/AddNamedGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct AddNamedGroupingPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for AddNamedGroupingPolicySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).add_named_grouping_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddNamedGroupingPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/RemoveGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveGroupingPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for RemoveGroupingPolicySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).remove_grouping_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveGroupingPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/RemoveNamedGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveNamedGroupingPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest>
                        for RemoveNamedGroupingPolicySvc<T>
                    {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).remove_named_grouping_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveNamedGroupingPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/RemoveFilteredGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveFilteredGroupingPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::FilteredPolicyRequest>
                        for RemoveFilteredGroupingPolicySvc<T>
                    {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FilteredPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).remove_filtered_grouping_policy(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveFilteredGroupingPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/RemoveFilteredNamedGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveFilteredNamedGroupingPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::FilteredPolicyRequest>
                        for RemoveFilteredNamedGroupingPolicySvc<T>
                    {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FilteredPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .remove_filtered_named_grouping_policy(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveFilteredNamedGroupingPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetGroupingPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::EmptyRequest> for GetGroupingPolicySvc<T> {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_grouping_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetGroupingPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetNamedGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetNamedGroupingPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for GetNamedGroupingPolicySvc<T> {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_named_grouping_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetNamedGroupingPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetFilteredGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetFilteredGroupingPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::FilteredPolicyRequest>
                        for GetFilteredGroupingPolicySvc<T>
                    {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FilteredPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_filtered_grouping_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFilteredGroupingPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetFilteredNamedGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetFilteredNamedGroupingPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::FilteredPolicyRequest>
                        for GetFilteredNamedGroupingPolicySvc<T>
                    {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FilteredPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_filtered_named_grouping_policy(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFilteredNamedGroupingPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetAllSubjects" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllSubjectsSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::EmptyRequest> for GetAllSubjectsSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_all_subjects(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllSubjectsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetAllNamedSubjects" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllNamedSubjectsSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::SimpleGetRequest> for GetAllNamedSubjectsSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SimpleGetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_all_named_subjects(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllNamedSubjectsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetAllObjects" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllObjectsSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::EmptyRequest> for GetAllObjectsSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_all_objects(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllObjectsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetAllNamedObjects" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllNamedObjectsSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::SimpleGetRequest> for GetAllNamedObjectsSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SimpleGetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_all_named_objects(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllNamedObjectsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetAllActions" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllActionsSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::EmptyRequest> for GetAllActionsSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_all_actions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllActionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetAllNamedActions" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllNamedActionsSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::SimpleGetRequest> for GetAllNamedActionsSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SimpleGetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_all_named_actions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllNamedActionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetAllRoles" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllRolesSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::EmptyRequest> for GetAllRolesSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EmptyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_all_roles(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllRolesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetAllNamedRoles" => {
                    #[allow(non_camel_case_types)]
                    struct GetAllNamedRolesSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::SimpleGetRequest> for GetAllNamedRolesSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SimpleGetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_all_named_roles(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAllNamedRolesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/HasPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct HasPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for HasPolicySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).has_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HasPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/HasNamedPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct HasNamedPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for HasNamedPolicySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).has_named_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HasNamedPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/HasGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct HasGroupingPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for HasGroupingPolicySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).has_grouping_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HasGroupingPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/HasNamedGroupingPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct HasNamedGroupingPolicySvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PolicyRequest> for HasNamedGroupingPolicySvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).has_named_grouping_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HasNamedGroupingPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetRolesForUser" => {
                    #[allow(non_camel_case_types)]
                    struct GetRolesForUserSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::UserRoleRequest> for GetRolesForUserSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UserRoleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_roles_for_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRolesForUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetImplicitRolesForUser" => {
                    #[allow(non_camel_case_types)]
                    struct GetImplicitRolesForUserSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::UserRoleRequest>
                        for GetImplicitRolesForUserSvc<T>
                    {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UserRoleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_implicit_roles_for_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetImplicitRolesForUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetUsersForRole" => {
                    #[allow(non_camel_case_types)]
                    struct GetUsersForRoleSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::UserRoleRequest> for GetUsersForRoleSvc<T> {
                        type Response = super::ArrayReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UserRoleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_users_for_role(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUsersForRoleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/HasRoleForUser" => {
                    #[allow(non_camel_case_types)]
                    struct HasRoleForUserSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::UserRoleRequest> for HasRoleForUserSvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UserRoleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).has_role_for_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HasRoleForUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/AddRoleForUser" => {
                    #[allow(non_camel_case_types)]
                    struct AddRoleForUserSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::UserRoleRequest> for AddRoleForUserSvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UserRoleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_role_for_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddRoleForUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/DeleteRoleForUser" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRoleForUserSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::UserRoleRequest> for DeleteRoleForUserSvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UserRoleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_role_for_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteRoleForUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/DeleteRolesForUser" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRolesForUserSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::UserRoleRequest> for DeleteRolesForUserSvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UserRoleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_roles_for_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteRolesForUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/DeleteUser" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteUserSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::UserRoleRequest> for DeleteUserSvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UserRoleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/DeleteRole" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRoleSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::UserRoleRequest> for DeleteRoleSvc<T> {
                        type Response = super::EmptyReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UserRoleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_role(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteRoleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetPermissionsForUser" => {
                    #[allow(non_camel_case_types)]
                    struct GetPermissionsForUserSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PermissionRequest>
                        for GetPermissionsForUserSvc<T>
                    {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PermissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_permissions_for_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPermissionsForUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/GetImplicitPermissionsForUser" => {
                    #[allow(non_camel_case_types)]
                    struct GetImplicitPermissionsForUserSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PermissionRequest>
                        for GetImplicitPermissionsForUserSvc<T>
                    {
                        type Response = super::Array2DReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PermissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_implicit_permissions_for_user(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetImplicitPermissionsForUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/DeletePermission" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePermissionSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PermissionRequest> for DeletePermissionSvc<T> {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PermissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_permission(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeletePermissionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/AddPermissionForUser" => {
                    #[allow(non_camel_case_types)]
                    struct AddPermissionForUserSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PermissionRequest>
                        for AddPermissionForUserSvc<T>
                    {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PermissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).add_permission_for_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddPermissionForUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/DeletePermissionForUser" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePermissionForUserSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PermissionRequest>
                        for DeletePermissionForUserSvc<T>
                    {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PermissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).delete_permission_for_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeletePermissionForUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/DeletePermissionsForUser" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePermissionsForUserSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PermissionRequest>
                        for DeletePermissionsForUserSvc<T>
                    {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PermissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).delete_permissions_for_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeletePermissionsForUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.Casbin/HasPermissionForUser" => {
                    #[allow(non_camel_case_types)]
                    struct HasPermissionForUserSvc<T: Casbin>(pub Arc<T>);
                    impl<T: Casbin> tonic::server::UnaryService<super::PermissionRequest>
                        for HasPermissionForUserSvc<T>
                    {
                        type Response = super::BoolReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PermissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).has_permission_for_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HasPermissionForUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Casbin> Clone for CasbinServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Casbin> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Casbin> tonic::server::NamedService for CasbinServer<T> {
        const NAME: &'static str = "proto.Casbin";
    }
}
