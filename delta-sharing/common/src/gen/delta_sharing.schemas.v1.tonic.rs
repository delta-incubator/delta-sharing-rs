// @generated
/// Generated server implementations.
pub mod schemas_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SchemasServiceServer.
    #[async_trait]
    pub trait SchemasService: Send + Sync + 'static {
        /** Gets an array of schemas for a catalog in the metastore. If the caller is the metastore
 admin or the owner of the parent catalog, all schemas for the catalog will be retrieved.
 Otherwise, only schemas owned by the caller (or for which the caller has the USE_SCHEMA privilege)
 will be retrieved. There is no guarantee of a specific ordering of the elements in the array.
*/
        async fn list_schemas(
            &self,
            request: tonic::Request<super::ListSchemasRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSchemasResponse>,
            tonic::Status,
        >;
        /** Creates a new schema for catalog in the Metatastore. The caller must be a metastore admin,
 or have the CREATE_SCHEMA privilege in the parent catalog.
*/
        async fn create_schema(
            &self,
            request: tonic::Request<super::CreateSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::SchemaInfo>, tonic::Status>;
        /** Gets the specified schema within the metastore.
 The caller must be a metastore admin, the owner of the schema,
 or a user that has the USE_SCHEMA privilege on the schema.
*/
        async fn get_schema(
            &self,
            request: tonic::Request<super::GetSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::SchemaInfo>, tonic::Status>;
        /** Updates a schema for a catalog. The caller must be the owner of the schema or a metastore admin.
 If the caller is a metastore admin, only the owner field can be changed in the update.
 If the name field must be updated, the caller must be a metastore admin or have the CREATE_SCHEMA
 privilege on the parent catalog.
*/
        async fn update_schema(
            &self,
            request: tonic::Request<super::UpdateSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::SchemaInfo>, tonic::Status>;
        /** Deletes the specified schema from the parent catalog. The caller must be the owner
 of the schema or an owner of the parent catalog.
*/
        async fn delete_schema(
            &self,
            request: tonic::Request<super::DeleteSchemaRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /** A schema (also called a database) is the second layer of Unity Catalog’s three-level namespace.
 A schema organizes tables, views and functions. To access (or list) a table or view in a schema,
 users must have the USE_SCHEMA data permission on the schema and its parent catalog, and they must
 have the SELECT permission on the table or view.
*/
    #[derive(Debug)]
    pub struct SchemasServiceServer<T: SchemasService> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: SchemasService> SchemasServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SchemasServiceServer<T>
    where
        T: SchemasService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/delta_sharing.schemas.v1.SchemasService/ListSchemas" => {
                    #[allow(non_camel_case_types)]
                    struct ListSchemasSvc<T: SchemasService>(pub Arc<T>);
                    impl<
                        T: SchemasService,
                    > tonic::server::UnaryService<super::ListSchemasRequest>
                    for ListSchemasSvc<T> {
                        type Response = super::ListSchemasResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListSchemasRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SchemasService>::list_schemas(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ListSchemasSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/delta_sharing.schemas.v1.SchemasService/CreateSchema" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSchemaSvc<T: SchemasService>(pub Arc<T>);
                    impl<
                        T: SchemasService,
                    > tonic::server::UnaryService<super::CreateSchemaRequest>
                    for CreateSchemaSvc<T> {
                        type Response = super::SchemaInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateSchemaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SchemasService>::create_schema(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = CreateSchemaSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/delta_sharing.schemas.v1.SchemasService/GetSchema" => {
                    #[allow(non_camel_case_types)]
                    struct GetSchemaSvc<T: SchemasService>(pub Arc<T>);
                    impl<
                        T: SchemasService,
                    > tonic::server::UnaryService<super::GetSchemaRequest>
                    for GetSchemaSvc<T> {
                        type Response = super::SchemaInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSchemaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SchemasService>::get_schema(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = GetSchemaSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/delta_sharing.schemas.v1.SchemasService/UpdateSchema" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSchemaSvc<T: SchemasService>(pub Arc<T>);
                    impl<
                        T: SchemasService,
                    > tonic::server::UnaryService<super::UpdateSchemaRequest>
                    for UpdateSchemaSvc<T> {
                        type Response = super::SchemaInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateSchemaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SchemasService>::update_schema(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = UpdateSchemaSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/delta_sharing.schemas.v1.SchemasService/DeleteSchema" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSchemaSvc<T: SchemasService>(pub Arc<T>);
                    impl<
                        T: SchemasService,
                    > tonic::server::UnaryService<super::DeleteSchemaRequest>
                    for DeleteSchemaSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteSchemaRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SchemasService>::delete_schema(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = DeleteSchemaSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", tonic::Code::Unimplemented as i32)
                                .header(
                                    http::header::CONTENT_TYPE,
                                    tonic::metadata::GRPC_CONTENT_TYPE,
                                )
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: SchemasService> Clone for SchemasServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: SchemasService> tonic::server::NamedService for SchemasServiceServer<T> {
        const NAME: &'static str = "delta_sharing.schemas.v1.SchemasService";
    }
}
