// @generated
/// Generated server implementations.
pub mod catalog_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CatalogServiceServer.
    #[async_trait]
    pub trait CatalogService: Send + Sync + 'static {
        /** Create a new catalog
*/
        async fn create_catalog(
            &self,
            request: tonic::Request<super::CreateCatalogRequest>,
        ) -> std::result::Result<tonic::Response<super::CatalogInfo>, tonic::Status>;
        /** Get a catalog
*/
        async fn get_catalog(
            &self,
            request: tonic::Request<super::GetCatalogRequest>,
        ) -> std::result::Result<tonic::Response<super::CatalogInfo>, tonic::Status>;
        /** List catalogs
*/
        async fn list_catalogs(
            &self,
            request: tonic::Request<super::ListCatalogsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCatalogsResponse>,
            tonic::Status,
        >;
        /** Delete a catalog
*/
        async fn delete_catalog(
            &self,
            request: tonic::Request<super::DeleteCatalogRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** Update a catalog
*/
        async fn update_catalog(
            &self,
            request: tonic::Request<super::UpdateCatalogRequest>,
        ) -> std::result::Result<tonic::Response<super::CatalogInfo>, tonic::Status>;
        /** Create a new Schema
*/
        async fn create_schema(
            &self,
            request: tonic::Request<super::CreateSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::SchemaInfo>, tonic::Status>;
        /** Delete a Schema
*/
        async fn delete_schema(
            &self,
            request: tonic::Request<super::DeleteSchemaRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        /** Get a Schema
*/
        async fn get_schema(
            &self,
            request: tonic::Request<super::GetSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::SchemaInfo>, tonic::Status>;
        /** List Schemas
*/
        async fn list_schemas(
            &self,
            request: tonic::Request<super::ListSchemasRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListSchemasResponse>,
            tonic::Status,
        >;
        /** Update a Schema
*/
        async fn update_schema(
            &self,
            request: tonic::Request<super::UpdateSchemaRequest>,
        ) -> std::result::Result<tonic::Response<super::SchemaInfo>, tonic::Status>;
    }
    /** Manage catalogs and schemas in the service.
*/
    #[derive(Debug)]
    pub struct CatalogServiceServer<T: CatalogService> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: CatalogService> CatalogServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CatalogServiceServer<T>
    where
        T: CatalogService,
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
                "/delta_sharing.catalog.v1.CatalogService/CreateCatalog" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCatalogSvc<T: CatalogService>(pub Arc<T>);
                    impl<
                        T: CatalogService,
                    > tonic::server::UnaryService<super::CreateCatalogRequest>
                    for CreateCatalogSvc<T> {
                        type Response = super::CatalogInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCatalogRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CatalogService>::create_catalog(&inner, request).await
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
                        let method = CreateCatalogSvc(inner);
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
                "/delta_sharing.catalog.v1.CatalogService/GetCatalog" => {
                    #[allow(non_camel_case_types)]
                    struct GetCatalogSvc<T: CatalogService>(pub Arc<T>);
                    impl<
                        T: CatalogService,
                    > tonic::server::UnaryService<super::GetCatalogRequest>
                    for GetCatalogSvc<T> {
                        type Response = super::CatalogInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCatalogRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CatalogService>::get_catalog(&inner, request).await
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
                        let method = GetCatalogSvc(inner);
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
                "/delta_sharing.catalog.v1.CatalogService/ListCatalogs" => {
                    #[allow(non_camel_case_types)]
                    struct ListCatalogsSvc<T: CatalogService>(pub Arc<T>);
                    impl<
                        T: CatalogService,
                    > tonic::server::UnaryService<super::ListCatalogsRequest>
                    for ListCatalogsSvc<T> {
                        type Response = super::ListCatalogsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCatalogsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CatalogService>::list_catalogs(&inner, request).await
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
                        let method = ListCatalogsSvc(inner);
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
                "/delta_sharing.catalog.v1.CatalogService/DeleteCatalog" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCatalogSvc<T: CatalogService>(pub Arc<T>);
                    impl<
                        T: CatalogService,
                    > tonic::server::UnaryService<super::DeleteCatalogRequest>
                    for DeleteCatalogSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteCatalogRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CatalogService>::delete_catalog(&inner, request).await
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
                        let method = DeleteCatalogSvc(inner);
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
                "/delta_sharing.catalog.v1.CatalogService/UpdateCatalog" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCatalogSvc<T: CatalogService>(pub Arc<T>);
                    impl<
                        T: CatalogService,
                    > tonic::server::UnaryService<super::UpdateCatalogRequest>
                    for UpdateCatalogSvc<T> {
                        type Response = super::CatalogInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCatalogRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CatalogService>::update_catalog(&inner, request).await
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
                        let method = UpdateCatalogSvc(inner);
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
                "/delta_sharing.catalog.v1.CatalogService/CreateSchema" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSchemaSvc<T: CatalogService>(pub Arc<T>);
                    impl<
                        T: CatalogService,
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
                                <T as CatalogService>::create_schema(&inner, request).await
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
                "/delta_sharing.catalog.v1.CatalogService/DeleteSchema" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSchemaSvc<T: CatalogService>(pub Arc<T>);
                    impl<
                        T: CatalogService,
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
                                <T as CatalogService>::delete_schema(&inner, request).await
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
                "/delta_sharing.catalog.v1.CatalogService/GetSchema" => {
                    #[allow(non_camel_case_types)]
                    struct GetSchemaSvc<T: CatalogService>(pub Arc<T>);
                    impl<
                        T: CatalogService,
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
                                <T as CatalogService>::get_schema(&inner, request).await
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
                "/delta_sharing.catalog.v1.CatalogService/ListSchemas" => {
                    #[allow(non_camel_case_types)]
                    struct ListSchemasSvc<T: CatalogService>(pub Arc<T>);
                    impl<
                        T: CatalogService,
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
                                <T as CatalogService>::list_schemas(&inner, request).await
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
                "/delta_sharing.catalog.v1.CatalogService/UpdateSchema" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSchemaSvc<T: CatalogService>(pub Arc<T>);
                    impl<
                        T: CatalogService,
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
                                <T as CatalogService>::update_schema(&inner, request).await
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
    impl<T: CatalogService> Clone for CatalogServiceServer<T> {
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
    impl<T: CatalogService> tonic::server::NamedService for CatalogServiceServer<T> {
        const NAME: &'static str = "delta_sharing.catalog.v1.CatalogService";
    }
}
