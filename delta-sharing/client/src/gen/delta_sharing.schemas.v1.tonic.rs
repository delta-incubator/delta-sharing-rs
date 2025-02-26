// @generated
/// Generated client implementations.
pub mod schemas_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** A schema (also called a database) is the second layer of Unity Catalog’s three-level namespace.
 A schema organizes tables, views and functions. To access (or list) a table or view in a schema,
 users must have the USE_SCHEMA data permission on the schema and its parent catalog, and they must
 have the SELECT permission on the table or view.
*/
    #[derive(Debug, Clone)]
    pub struct SchemasServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SchemasServiceClient<tonic::transport::Channel> {
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
    impl<T> SchemasServiceClient<T>
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
        ) -> SchemasServiceClient<InterceptedService<T, F>>
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
            SchemasServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /** Creates a new schema for catalog in the Metatastore. The caller must be a metastore admin,
 or have the CREATE_SCHEMA privilege in the parent catalog.
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
                "/delta_sharing.schemas.v1.SchemasService/CreateSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.schemas.v1.SchemasService",
                        "CreateSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Deletes the specified schema from the parent catalog. The caller must be the owner
 of the schema or an owner of the parent catalog.
*/
        pub async fn delete_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSchemaRequest>,
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
                "/delta_sharing.schemas.v1.SchemasService/DeleteSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.schemas.v1.SchemasService",
                        "DeleteSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Gets the specified schema within the metastore.
 The caller must be a metastore admin, the owner of the schema,
 or a user that has the USE_SCHEMA privilege on the schema.
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
                "/delta_sharing.schemas.v1.SchemasService/GetSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.schemas.v1.SchemasService",
                        "GetSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Gets an array of schemas for a catalog in the metastore. If the caller is the metastore
 admin or the owner of the parent catalog, all schemas for the catalog will be retrieved.
 Otherwise, only schemas owned by the caller (or for which the caller has the USE_SCHEMA privilege)
 will be retrieved. There is no guarantee of a specific ordering of the elements in the array.
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
                "/delta_sharing.schemas.v1.SchemasService/ListSchemas",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.schemas.v1.SchemasService",
                        "ListSchemas",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Updates a schema for a catalog. The caller must be the owner of the schema or a metastore admin.
 If the caller is a metastore admin, only the owner field can be changed in the update.
 If the name field must be updated, the caller must be a metastore admin or have the CREATE_SCHEMA
 privilege on the parent catalog.
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
                "/delta_sharing.schemas.v1.SchemasService/UpdateSchema",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.schemas.v1.SchemasService",
                        "UpdateSchema",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
