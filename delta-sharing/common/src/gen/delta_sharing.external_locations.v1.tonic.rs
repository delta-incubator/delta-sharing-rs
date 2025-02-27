// @generated
/// Generated server implementations.
pub mod external_locations_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ExternalLocationsServiceServer.
    #[async_trait]
    pub trait ExternalLocationsService: Send + Sync + 'static {
        /** List external locations
*/
        async fn list_external_locations(
            &self,
            request: tonic::Request<super::ListExternalLocationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListExternalLocationsResponse>,
            tonic::Status,
        >;
        /** Create a new external location
*/
        async fn create_external_location(
            &self,
            request: tonic::Request<super::CreateExternalLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExternalLocationInfo>,
            tonic::Status,
        >;
        /** Get an external location
*/
        async fn get_external_location(
            &self,
            request: tonic::Request<super::GetExternalLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExternalLocationInfo>,
            tonic::Status,
        >;
        /** Update an external location
*/
        async fn update_external_location(
            &self,
            request: tonic::Request<super::UpdateExternalLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ExternalLocationInfo>,
            tonic::Status,
        >;
        /** Delete an external location
*/
        async fn delete_external_location(
            &self,
            request: tonic::Request<super::DeleteExternalLocationRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    ///
    #[derive(Debug)]
    pub struct ExternalLocationsServiceServer<T: ExternalLocationsService> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: ExternalLocationsService> ExternalLocationsServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for ExternalLocationsServiceServer<T>
    where
        T: ExternalLocationsService,
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
                "/delta_sharing.external_locations.v1.ExternalLocationsService/ListExternalLocations" => {
                    #[allow(non_camel_case_types)]
                    struct ListExternalLocationsSvc<T: ExternalLocationsService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ExternalLocationsService,
                    > tonic::server::UnaryService<super::ListExternalLocationsRequest>
                    for ListExternalLocationsSvc<T> {
                        type Response = super::ListExternalLocationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListExternalLocationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ExternalLocationsService>::list_external_locations(
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
                        let method = ListExternalLocationsSvc(inner);
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
                "/delta_sharing.external_locations.v1.ExternalLocationsService/CreateExternalLocation" => {
                    #[allow(non_camel_case_types)]
                    struct CreateExternalLocationSvc<T: ExternalLocationsService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ExternalLocationsService,
                    > tonic::server::UnaryService<super::CreateExternalLocationRequest>
                    for CreateExternalLocationSvc<T> {
                        type Response = super::ExternalLocationInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateExternalLocationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ExternalLocationsService>::create_external_location(
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
                        let method = CreateExternalLocationSvc(inner);
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
                "/delta_sharing.external_locations.v1.ExternalLocationsService/GetExternalLocation" => {
                    #[allow(non_camel_case_types)]
                    struct GetExternalLocationSvc<T: ExternalLocationsService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ExternalLocationsService,
                    > tonic::server::UnaryService<super::GetExternalLocationRequest>
                    for GetExternalLocationSvc<T> {
                        type Response = super::ExternalLocationInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetExternalLocationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ExternalLocationsService>::get_external_location(
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
                        let method = GetExternalLocationSvc(inner);
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
                "/delta_sharing.external_locations.v1.ExternalLocationsService/UpdateExternalLocation" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateExternalLocationSvc<T: ExternalLocationsService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ExternalLocationsService,
                    > tonic::server::UnaryService<super::UpdateExternalLocationRequest>
                    for UpdateExternalLocationSvc<T> {
                        type Response = super::ExternalLocationInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateExternalLocationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ExternalLocationsService>::update_external_location(
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
                        let method = UpdateExternalLocationSvc(inner);
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
                "/delta_sharing.external_locations.v1.ExternalLocationsService/DeleteExternalLocation" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteExternalLocationSvc<T: ExternalLocationsService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ExternalLocationsService,
                    > tonic::server::UnaryService<super::DeleteExternalLocationRequest>
                    for DeleteExternalLocationSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteExternalLocationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as ExternalLocationsService>::delete_external_location(
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
                        let method = DeleteExternalLocationSvc(inner);
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
    impl<T: ExternalLocationsService> Clone for ExternalLocationsServiceServer<T> {
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
    impl<T: ExternalLocationsService> tonic::server::NamedService
    for ExternalLocationsServiceServer<T> {
        const NAME: &'static str = "delta_sharing.external_locations.v1.ExternalLocationsService";
    }
}
