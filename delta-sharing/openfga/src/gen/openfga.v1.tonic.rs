// @generated
/// Generated client implementations.
pub mod open_fga_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OpenFgaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OpenFgaServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OpenFgaServiceClient<T>
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
        ) -> OpenFgaServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            OpenFgaServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn read(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadRequest>,
        ) -> std::result::Result<tonic::Response<super::ReadResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/Read",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "Read"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn write(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteRequest>,
        ) -> std::result::Result<tonic::Response<super::WriteResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/Write",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "Write"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn check(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckRequest>,
        ) -> std::result::Result<tonic::Response<super::CheckResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/Check",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "Check"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_check(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCheckRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchCheckResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/BatchCheck",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "BatchCheck"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn expand(
            &mut self,
            request: impl tonic::IntoRequest<super::ExpandRequest>,
        ) -> std::result::Result<tonic::Response<super::ExpandResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/Expand",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "Expand"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_authorization_models(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadAuthorizationModelsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadAuthorizationModelsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/ReadAuthorizationModels",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "openfga.v1.OpenFGAService",
                        "ReadAuthorizationModels",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_authorization_model(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadAuthorizationModelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadAuthorizationModelResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/ReadAuthorizationModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "openfga.v1.OpenFGAService",
                        "ReadAuthorizationModel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn write_authorization_model(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteAuthorizationModelRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WriteAuthorizationModelResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/WriteAuthorizationModel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "openfga.v1.OpenFGAService",
                        "WriteAuthorizationModel",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn write_assertions(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteAssertionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WriteAssertionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/WriteAssertions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "WriteAssertions"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_assertions(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadAssertionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadAssertionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/ReadAssertions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "ReadAssertions"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn read_changes(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadChangesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReadChangesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/ReadChanges",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "ReadChanges"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_store(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateStoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateStoreResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/CreateStore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "CreateStore"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_store(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateStoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateStoreResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/UpdateStore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "UpdateStore"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_store(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteStoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteStoreResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/DeleteStore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "DeleteStore"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_store(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStoreRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStoreResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/GetStore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "GetStore"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_stores(
            &mut self,
            request: impl tonic::IntoRequest<super::ListStoresRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListStoresResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/ListStores",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "ListStores"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn streamed_list_objects(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamedListObjectsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::StreamedListObjectsResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/StreamedListObjects",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("openfga.v1.OpenFGAService", "StreamedListObjects"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn list_objects(
            &mut self,
            request: impl tonic::IntoRequest<super::ListObjectsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListObjectsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/ListObjects",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "ListObjects"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_users(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUsersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUsersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/openfga.v1.OpenFGAService/ListUsers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("openfga.v1.OpenFGAService", "ListUsers"));
            self.inner.unary(req, path, codec).await
        }
    }
}
