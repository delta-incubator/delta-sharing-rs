// @generated
/// Generated client implementations.
pub mod catalog_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    ///
    #[derive(Debug, Clone)]
    pub struct CatalogServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CatalogServiceClient<tonic::transport::Channel> {
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
    impl<T> CatalogServiceClient<T>
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
        ) -> CatalogServiceClient<InterceptedService<T, F>>
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
            CatalogServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /** Create a new catalog
*/
        pub async fn create_catalog(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCatalogRequest>,
        ) -> std::result::Result<tonic::Response<super::CatalogInfo>, tonic::Status> {
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
                "/delta_sharing.catalog.v1.CatalogService/CreateCatalog",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.catalog.v1.CatalogService",
                        "CreateCatalog",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Get a catalog
*/
        pub async fn get_catalog(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCatalogRequest>,
        ) -> std::result::Result<tonic::Response<super::CatalogInfo>, tonic::Status> {
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
                "/delta_sharing.catalog.v1.CatalogService/GetCatalog",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.catalog.v1.CatalogService",
                        "GetCatalog",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** List catalogs
*/
        pub async fn list_catalogs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCatalogsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCatalogsResponse>,
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
                "/delta_sharing.catalog.v1.CatalogService/ListCatalogs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.catalog.v1.CatalogService",
                        "ListCatalogs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Delete a catalog
*/
        pub async fn delete_catalog(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCatalogRequest>,
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
                "/delta_sharing.catalog.v1.CatalogService/DeleteCatalog",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.catalog.v1.CatalogService",
                        "DeleteCatalog",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Update a catalog
*/
        pub async fn update_catalog(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCatalogRequest>,
        ) -> std::result::Result<tonic::Response<super::CatalogInfo>, tonic::Status> {
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
                "/delta_sharing.catalog.v1.CatalogService/UpdateCatalog",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.catalog.v1.CatalogService",
                        "UpdateCatalog",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Create a new Schema
*/
        pub async fn create_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::SchemaInfo>, tonic::Status> {
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
                "/delta_sharing.catalog.v1.CatalogService/CreateSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.catalog.v1.CatalogService",
                        "CreateSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Delete a Schema
*/
        pub async fn delete_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::SchemaInfo>, tonic::Status> {
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
                "/delta_sharing.catalog.v1.CatalogService/DeleteSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.catalog.v1.CatalogService",
                        "DeleteSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Get a Schema
*/
        pub async fn get_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::SchemaInfo>, tonic::Status> {
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
                "/delta_sharing.catalog.v1.CatalogService/GetSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.catalog.v1.CatalogService",
                        "GetSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** List Schemas
*/
        pub async fn list_schemas(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSchemasRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSchemasResponse>,
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
                "/delta_sharing.catalog.v1.CatalogService/ListSchemas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.catalog.v1.CatalogService",
                        "ListSchemas",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Update a Schema
*/
        pub async fn update_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::SchemaInfo>, tonic::Status> {
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
                "/delta_sharing.catalog.v1.CatalogService/UpdateSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.catalog.v1.CatalogService",
                        "UpdateSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
