// @generated
/// Generated client implementations.
pub mod catalogs_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Manage catalogs and schemas in the service.
*/
    #[derive(Debug, Clone)]
    pub struct CatalogsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CatalogsServiceClient<tonic::transport::Channel> {
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
    impl<T> CatalogsServiceClient<T>
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
        ) -> CatalogsServiceClient<InterceptedService<T, F>>
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
            CatalogsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /** List catalogs

 Gets an array of catalogs in the metastore. If the caller is the metastore admin,
 all catalogs will be retrieved. Otherwise, only catalogs owned by the caller
 (or for which the caller has the USE_CATALOG privilege) will be retrieved.
 There is no guarantee of a specific ordering of the elements in the array.
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
                "/delta_sharing.catalogs.v1.CatalogsService/ListCatalogs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.catalogs.v1.CatalogsService",
                        "ListCatalogs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Create a new catalog

 Creates a new catalog instance in the parent metastore if the caller
 is a metastore admin or has the CREATE_CATALOG privilege.
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
                "/delta_sharing.catalogs.v1.CatalogsService/CreateCatalog",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.catalogs.v1.CatalogsService",
                        "CreateCatalog",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Get a catalog

 Gets the specified catalog in a metastore. The caller must be a metastore admin,
 the owner of the catalog, or a user that has the USE_CATALOG privilege set for their account.
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
                "/delta_sharing.catalogs.v1.CatalogsService/GetCatalog",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.catalogs.v1.CatalogsService",
                        "GetCatalog",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Delete a catalog

 Deletes the catalog that matches the supplied name. The caller must be a metastore admin or the owner of the catalog.
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
                "/delta_sharing.catalogs.v1.CatalogsService/DeleteCatalog",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.catalogs.v1.CatalogsService",
                        "DeleteCatalog",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Update a catalog

 Updates the catalog that matches the supplied name. The caller must be either
 the owner of the catalog, or a metastore admin (when changing the owner field of the catalog).
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
                "/delta_sharing.catalogs.v1.CatalogsService/UpdateCatalog",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.catalogs.v1.CatalogsService",
                        "UpdateCatalog",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
