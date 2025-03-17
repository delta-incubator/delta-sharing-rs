// @generated
/// Generated server implementations.
pub mod tables_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with TablesServiceServer.
    #[async_trait]
    pub trait TablesService: Send + Sync + 'static {
        /** Gets an array of summaries for tables for a schema and catalog within the metastore. The table summaries returned are either:
 - summaries for tables (within the current metastore and parent catalog and schema), when the user is a metastore admin, or:
 - summaries for tables and schemas (within the current metastore and parent catalog) for which the user has ownership or the
   SELECT privilege on the table and ownership or USE_SCHEMA privilege on the schema, provided that the user also has ownership
   or the USE_CATALOG privilege on the parent catalog.

 There is no guarantee of a specific ordering of the elements in the array.
*/
        async fn list_table_summaries(
            &self,
            request: tonic::Request<super::ListTableSummariesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTableSummariesResponse>,
            tonic::Status,
        >;
        /** Gets an array of all tables for the current metastore under the parent catalog and schema.

 The caller must be a metastore admin or an owner of (or have the SELECT privilege on) the table.
 For the latter case, the caller must also be the owner or have the USE_CATALOG privilege on the
 parent catalog and the USE_SCHEMA privilege on the parent schema. There is no guarantee of a
 specific ordering of the elements in the array.
*/
        async fn list_tables(
            &self,
            request: tonic::Request<super::ListTablesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTablesResponse>,
            tonic::Status,
        >;
        /** Create a table
*/
        async fn create_table(
            &self,
            request: tonic::Request<super::CreateTableRequest>,
        ) -> std::result::Result<tonic::Response<super::TableInfo>, tonic::Status>;
        /** Get a table
*/
        async fn get_table(
            &self,
            request: tonic::Request<super::GetTableRequest>,
        ) -> std::result::Result<tonic::Response<super::TableInfo>, tonic::Status>;
        /** Get boolean reflecting if table exists
*/
        async fn get_table_exists(
            &self,
            request: tonic::Request<super::GetTableExistsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTableExistsResponse>,
            tonic::Status,
        >;
        /** Delete a table
*/
        async fn delete_table(
            &self,
            request: tonic::Request<super::DeleteTableRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct TablesServiceServer<T: TablesService> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: TablesService> TablesServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TablesServiceServer<T>
    where
        T: TablesService,
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
                "/delta_sharing.tables.v1.TablesService/ListTableSummaries" => {
                    #[allow(non_camel_case_types)]
                    struct ListTableSummariesSvc<T: TablesService>(pub Arc<T>);
                    impl<
                        T: TablesService,
                    > tonic::server::UnaryService<super::ListTableSummariesRequest>
                    for ListTableSummariesSvc<T> {
                        type Response = super::ListTableSummariesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTableSummariesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TablesService>::list_table_summaries(&inner, request)
                                    .await
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
                        let method = ListTableSummariesSvc(inner);
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
                "/delta_sharing.tables.v1.TablesService/ListTables" => {
                    #[allow(non_camel_case_types)]
                    struct ListTablesSvc<T: TablesService>(pub Arc<T>);
                    impl<
                        T: TablesService,
                    > tonic::server::UnaryService<super::ListTablesRequest>
                    for ListTablesSvc<T> {
                        type Response = super::ListTablesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTablesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TablesService>::list_tables(&inner, request).await
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
                        let method = ListTablesSvc(inner);
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
                "/delta_sharing.tables.v1.TablesService/CreateTable" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTableSvc<T: TablesService>(pub Arc<T>);
                    impl<
                        T: TablesService,
                    > tonic::server::UnaryService<super::CreateTableRequest>
                    for CreateTableSvc<T> {
                        type Response = super::TableInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTableRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TablesService>::create_table(&inner, request).await
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
                        let method = CreateTableSvc(inner);
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
                "/delta_sharing.tables.v1.TablesService/GetTable" => {
                    #[allow(non_camel_case_types)]
                    struct GetTableSvc<T: TablesService>(pub Arc<T>);
                    impl<
                        T: TablesService,
                    > tonic::server::UnaryService<super::GetTableRequest>
                    for GetTableSvc<T> {
                        type Response = super::TableInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTableRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TablesService>::get_table(&inner, request).await
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
                        let method = GetTableSvc(inner);
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
                "/delta_sharing.tables.v1.TablesService/GetTableExists" => {
                    #[allow(non_camel_case_types)]
                    struct GetTableExistsSvc<T: TablesService>(pub Arc<T>);
                    impl<
                        T: TablesService,
                    > tonic::server::UnaryService<super::GetTableExistsRequest>
                    for GetTableExistsSvc<T> {
                        type Response = super::GetTableExistsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTableExistsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TablesService>::get_table_exists(&inner, request)
                                    .await
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
                        let method = GetTableExistsSvc(inner);
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
                "/delta_sharing.tables.v1.TablesService/DeleteTable" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTableSvc<T: TablesService>(pub Arc<T>);
                    impl<
                        T: TablesService,
                    > tonic::server::UnaryService<super::DeleteTableRequest>
                    for DeleteTableSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteTableRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as TablesService>::delete_table(&inner, request).await
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
                        let method = DeleteTableSvc(inner);
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
    impl<T: TablesService> Clone for TablesServiceServer<T> {
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
    impl<T: TablesService> tonic::server::NamedService for TablesServiceServer<T> {
        const NAME: &'static str = "delta_sharing.tables.v1.TablesService";
    }
}
