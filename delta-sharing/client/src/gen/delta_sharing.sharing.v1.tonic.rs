// @generated
/// Generated client implementations.
pub mod delta_sharing_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DeltaSharingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DeltaSharingServiceClient<tonic::transport::Channel> {
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
    impl<T> DeltaSharingServiceClient<T>
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
        ) -> DeltaSharingServiceClient<InterceptedService<T, F>>
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
            DeltaSharingServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_shares(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSharesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSharesResponse>,
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
                "/delta_sharing.sharing.v1.DeltaSharingService/ListShares",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.sharing.v1.DeltaSharingService",
                        "ListShares",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_share(
            &mut self,
            request: impl tonic::IntoRequest<super::GetShareRequest>,
        ) -> std::result::Result<tonic::Response<super::Share>, tonic::Status> {
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
                "/delta_sharing.sharing.v1.DeltaSharingService/GetShare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.sharing.v1.DeltaSharingService",
                        "GetShare",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_sharing_schemas(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSharingSchemasRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSharingSchemasResponse>,
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
                "/delta_sharing.sharing.v1.DeltaSharingService/ListSharingSchemas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.sharing.v1.DeltaSharingService",
                        "ListSharingSchemas",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_schema_tables(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSchemaTablesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSchemaTablesResponse>,
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
                "/delta_sharing.sharing.v1.DeltaSharingService/ListSchemaTables",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.sharing.v1.DeltaSharingService",
                        "ListSchemaTables",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_share_tables(
            &mut self,
            request: impl tonic::IntoRequest<super::ListShareTablesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListShareTablesResponse>,
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
                "/delta_sharing.sharing.v1.DeltaSharingService/ListShareTables",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.sharing.v1.DeltaSharingService",
                        "ListShareTables",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_table_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTableVersionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTableVersionResponse>,
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
                "/delta_sharing.sharing.v1.DeltaSharingService/GetTableVersion",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.sharing.v1.DeltaSharingService",
                        "GetTableVersion",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_table_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTableMetadataRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryResponse>, tonic::Status> {
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
                "/delta_sharing.sharing.v1.DeltaSharingService/GetTableMetadata",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.sharing.v1.DeltaSharingService",
                        "GetTableMetadata",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod delta_sharing_extension_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Services exposing the unofficial APIs for Delta Sharing.
 Mainly used to manage resoureces exposed by the Delta Sharing service.
*/
    #[derive(Debug, Clone)]
    pub struct DeltaSharingExtensionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DeltaSharingExtensionServiceClient<tonic::transport::Channel> {
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
    impl<T> DeltaSharingExtensionServiceClient<T>
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
        ) -> DeltaSharingExtensionServiceClient<InterceptedService<T, F>>
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
            DeltaSharingExtensionServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /** Create a new Share in the Delta Sharing service.
*/
        pub async fn create_share(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateShareRequest>,
        ) -> std::result::Result<tonic::Response<super::ShareInfo>, tonic::Status> {
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
                "/delta_sharing.sharing.v1.DeltaSharingExtensionService/CreateShare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.sharing.v1.DeltaSharingExtensionService",
                        "CreateShare",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Delete a Share in the Delta Sharing service.
*/
        pub async fn delete_share(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteShareRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status> {
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
                "/delta_sharing.sharing.v1.DeltaSharingExtensionService/DeleteShare",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.sharing.v1.DeltaSharingExtensionService",
                        "DeleteShare",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Create a new Schema in the Delta Sharing service.
*/
        pub async fn create_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSharingSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SharingSchemaInfo>,
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
                "/delta_sharing.sharing.v1.DeltaSharingExtensionService/CreateSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.sharing.v1.DeltaSharingExtensionService",
                        "CreateSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Delete a Schema in the Delta Sharing service.
*/
        pub async fn delete_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSharingSchemaRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SharingSchemaInfo>,
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
                "/delta_sharing.sharing.v1.DeltaSharingExtensionService/DeleteSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.sharing.v1.DeltaSharingExtensionService",
                        "DeleteSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
