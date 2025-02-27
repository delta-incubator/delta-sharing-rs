// @generated
/// Generated server implementations.
pub mod recipients_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RecipientsServiceServer.
    #[async_trait]
    pub trait RecipientsService: Send + Sync + 'static {
        /** List recipients.
*/
        async fn list_recipients(
            &self,
            request: tonic::Request<super::ListRecipientsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRecipientsResponse>,
            tonic::Status,
        >;
        /** Create a new recipient.
*/
        async fn create_recipient(
            &self,
            request: tonic::Request<super::CreateRecipientRequest>,
        ) -> std::result::Result<tonic::Response<super::RecipientInfo>, tonic::Status>;
        /** Get a recipient by name.
*/
        async fn get_recipient(
            &self,
            request: tonic::Request<super::GetRecipientRequest>,
        ) -> std::result::Result<tonic::Response<super::RecipientInfo>, tonic::Status>;
        /** Update a recipient.
*/
        async fn update_recipient(
            &self,
            request: tonic::Request<super::UpdateRecipientRequest>,
        ) -> std::result::Result<tonic::Response<super::RecipientInfo>, tonic::Status>;
        /** Delete a recipient.
*/
        async fn delete_recipient(
            &self,
            request: tonic::Request<super::DeleteRecipientRequest>,
        ) -> std::result::Result<tonic::Response<()>, tonic::Status>;
    }
    /** Recipients

 A recipient is an object you create using recipients/create to represent an organization which
 you want to allow access shares. when you create a recipient object, Unity Catalog generates an
 activation link you can send to the recipient. The recipient follows the activation link to download
 the credential file, and then uses the credential file to establish a secure connection to receive
 the shared data. This sharing mode is called open sharing.
*/
    #[derive(Debug)]
    pub struct RecipientsServiceServer<T: RecipientsService> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: RecipientsService> RecipientsServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RecipientsServiceServer<T>
    where
        T: RecipientsService,
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
                "/delta_sharing.recipients.v1.RecipientsService/ListRecipients" => {
                    #[allow(non_camel_case_types)]
                    struct ListRecipientsSvc<T: RecipientsService>(pub Arc<T>);
                    impl<
                        T: RecipientsService,
                    > tonic::server::UnaryService<super::ListRecipientsRequest>
                    for ListRecipientsSvc<T> {
                        type Response = super::ListRecipientsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRecipientsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RecipientsService>::list_recipients(&inner, request)
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
                        let method = ListRecipientsSvc(inner);
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
                "/delta_sharing.recipients.v1.RecipientsService/CreateRecipient" => {
                    #[allow(non_camel_case_types)]
                    struct CreateRecipientSvc<T: RecipientsService>(pub Arc<T>);
                    impl<
                        T: RecipientsService,
                    > tonic::server::UnaryService<super::CreateRecipientRequest>
                    for CreateRecipientSvc<T> {
                        type Response = super::RecipientInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRecipientRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RecipientsService>::create_recipient(&inner, request)
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
                        let method = CreateRecipientSvc(inner);
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
                "/delta_sharing.recipients.v1.RecipientsService/GetRecipient" => {
                    #[allow(non_camel_case_types)]
                    struct GetRecipientSvc<T: RecipientsService>(pub Arc<T>);
                    impl<
                        T: RecipientsService,
                    > tonic::server::UnaryService<super::GetRecipientRequest>
                    for GetRecipientSvc<T> {
                        type Response = super::RecipientInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRecipientRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RecipientsService>::get_recipient(&inner, request)
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
                        let method = GetRecipientSvc(inner);
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
                "/delta_sharing.recipients.v1.RecipientsService/UpdateRecipient" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateRecipientSvc<T: RecipientsService>(pub Arc<T>);
                    impl<
                        T: RecipientsService,
                    > tonic::server::UnaryService<super::UpdateRecipientRequest>
                    for UpdateRecipientSvc<T> {
                        type Response = super::RecipientInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRecipientRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RecipientsService>::update_recipient(&inner, request)
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
                        let method = UpdateRecipientSvc(inner);
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
                "/delta_sharing.recipients.v1.RecipientsService/DeleteRecipient" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRecipientSvc<T: RecipientsService>(pub Arc<T>);
                    impl<
                        T: RecipientsService,
                    > tonic::server::UnaryService<super::DeleteRecipientRequest>
                    for DeleteRecipientSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRecipientRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as RecipientsService>::delete_recipient(&inner, request)
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
                        let method = DeleteRecipientSvc(inner);
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
    impl<T: RecipientsService> Clone for RecipientsServiceServer<T> {
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
    impl<T: RecipientsService> tonic::server::NamedService
    for RecipientsServiceServer<T> {
        const NAME: &'static str = "delta_sharing.recipients.v1.RecipientsService";
    }
}
