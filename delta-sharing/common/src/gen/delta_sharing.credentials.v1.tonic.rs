// @generated
/// Generated server implementations.
pub mod credential_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CredentialServiceServer.
    #[async_trait]
    pub trait CredentialService: Send + Sync + 'static {
        ///
        async fn create_credential(
            &self,
            request: tonic::Request<super::CreateCredentialRequest>,
        ) -> std::result::Result<tonic::Response<super::Credential>, tonic::Status>;
        ///
        async fn get_credential(
            &self,
            request: tonic::Request<super::GetCredentialRequest>,
        ) -> std::result::Result<tonic::Response<super::Credential>, tonic::Status>;
        ///
        async fn delete_credential(
            &self,
            request: tonic::Request<super::DeleteCredentialRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
        ///
        async fn create_storage_location(
            &self,
            request: tonic::Request<super::CreateStorageLocationRequest>,
        ) -> std::result::Result<tonic::Response<super::StorageLocation>, tonic::Status>;
        ///
        async fn get_storage_location(
            &self,
            request: tonic::Request<super::GetStorageLocationRequest>,
        ) -> std::result::Result<tonic::Response<super::StorageLocation>, tonic::Status>;
        ///
        async fn delete_storage_location(
            &self,
            request: tonic::Request<super::DeleteStorageLocationRequest>,
        ) -> std::result::Result<tonic::Response<super::StorageLocation>, tonic::Status>;
        ///
        async fn list_storage_locations(
            &self,
            request: tonic::Request<super::ListStorageLocationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListStorageLocationsResponse>,
            tonic::Status,
        >;
    }
    /** Manage credentials to access external data sources and services
 as well as generate signed urls for the Delta Sharing service.
*/
    #[derive(Debug)]
    pub struct CredentialServiceServer<T: CredentialService> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: CredentialService> CredentialServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CredentialServiceServer<T>
    where
        T: CredentialService,
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
                "/delta_sharing.credentials.v1.CredentialService/CreateCredential" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCredentialSvc<T: CredentialService>(pub Arc<T>);
                    impl<
                        T: CredentialService,
                    > tonic::server::UnaryService<super::CreateCredentialRequest>
                    for CreateCredentialSvc<T> {
                        type Response = super::Credential;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCredentialRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CredentialService>::create_credential(&inner, request)
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
                        let method = CreateCredentialSvc(inner);
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
                "/delta_sharing.credentials.v1.CredentialService/GetCredential" => {
                    #[allow(non_camel_case_types)]
                    struct GetCredentialSvc<T: CredentialService>(pub Arc<T>);
                    impl<
                        T: CredentialService,
                    > tonic::server::UnaryService<super::GetCredentialRequest>
                    for GetCredentialSvc<T> {
                        type Response = super::Credential;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCredentialRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CredentialService>::get_credential(&inner, request)
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
                        let method = GetCredentialSvc(inner);
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
                "/delta_sharing.credentials.v1.CredentialService/DeleteCredential" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCredentialSvc<T: CredentialService>(pub Arc<T>);
                    impl<
                        T: CredentialService,
                    > tonic::server::UnaryService<super::DeleteCredentialRequest>
                    for DeleteCredentialSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteCredentialRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CredentialService>::delete_credential(&inner, request)
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
                        let method = DeleteCredentialSvc(inner);
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
                "/delta_sharing.credentials.v1.CredentialService/CreateStorageLocation" => {
                    #[allow(non_camel_case_types)]
                    struct CreateStorageLocationSvc<T: CredentialService>(pub Arc<T>);
                    impl<
                        T: CredentialService,
                    > tonic::server::UnaryService<super::CreateStorageLocationRequest>
                    for CreateStorageLocationSvc<T> {
                        type Response = super::StorageLocation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateStorageLocationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CredentialService>::create_storage_location(
                                        &inner,
                                        request,
                                    )
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
                        let method = CreateStorageLocationSvc(inner);
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
                "/delta_sharing.credentials.v1.CredentialService/GetStorageLocation" => {
                    #[allow(non_camel_case_types)]
                    struct GetStorageLocationSvc<T: CredentialService>(pub Arc<T>);
                    impl<
                        T: CredentialService,
                    > tonic::server::UnaryService<super::GetStorageLocationRequest>
                    for GetStorageLocationSvc<T> {
                        type Response = super::StorageLocation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetStorageLocationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CredentialService>::get_storage_location(
                                        &inner,
                                        request,
                                    )
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
                        let method = GetStorageLocationSvc(inner);
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
                "/delta_sharing.credentials.v1.CredentialService/DeleteStorageLocation" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteStorageLocationSvc<T: CredentialService>(pub Arc<T>);
                    impl<
                        T: CredentialService,
                    > tonic::server::UnaryService<super::DeleteStorageLocationRequest>
                    for DeleteStorageLocationSvc<T> {
                        type Response = super::StorageLocation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteStorageLocationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CredentialService>::delete_storage_location(
                                        &inner,
                                        request,
                                    )
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
                        let method = DeleteStorageLocationSvc(inner);
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
                "/delta_sharing.credentials.v1.CredentialService/ListStorageLocations" => {
                    #[allow(non_camel_case_types)]
                    struct ListStorageLocationsSvc<T: CredentialService>(pub Arc<T>);
                    impl<
                        T: CredentialService,
                    > tonic::server::UnaryService<super::ListStorageLocationsRequest>
                    for ListStorageLocationsSvc<T> {
                        type Response = super::ListStorageLocationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListStorageLocationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as CredentialService>::list_storage_locations(
                                        &inner,
                                        request,
                                    )
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
                        let method = ListStorageLocationsSvc(inner);
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
    impl<T: CredentialService> Clone for CredentialServiceServer<T> {
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
    impl<T: CredentialService> tonic::server::NamedService
    for CredentialServiceServer<T> {
        const NAME: &'static str = "delta_sharing.credentials.v1.CredentialService";
    }
}
