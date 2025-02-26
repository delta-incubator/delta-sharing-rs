// @generated
/// Generated client implementations.
pub mod recipients_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Recipients

 A recipient is an object you create using recipients/create to represent an organization which
 you want to allow access shares. when you create a recipient object, Unity Catalog generates an
 activation link you can send to the recipient. The recipient follows the activation link to download
 the credential file, and then uses the credential file to establish a secure connection to receive
 the shared data. This sharing mode is called open sharing.
*/
    #[derive(Debug, Clone)]
    pub struct RecipientsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RecipientsServiceClient<tonic::transport::Channel> {
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
    impl<T> RecipientsServiceClient<T>
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
        ) -> RecipientsServiceClient<InterceptedService<T, F>>
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
            RecipientsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /** List recipients.
*/
        pub async fn list_recipients(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRecipientsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRecipientsResponse>,
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
                "/delta_sharing.recipients.v1.RecipientsService/ListRecipients",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.recipients.v1.RecipientsService",
                        "ListRecipients",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Create a new recipient.
*/
        pub async fn create_recipient(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRecipientRequest>,
        ) -> std::result::Result<tonic::Response<super::RecipientInfo>, tonic::Status> {
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
                "/delta_sharing.recipients.v1.RecipientsService/CreateRecipient",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.recipients.v1.RecipientsService",
                        "CreateRecipient",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Get a recipient by name.
*/
        pub async fn get_recipient(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRecipientRequest>,
        ) -> std::result::Result<tonic::Response<super::RecipientInfo>, tonic::Status> {
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
                "/delta_sharing.recipients.v1.RecipientsService/GetRecipient",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.recipients.v1.RecipientsService",
                        "GetRecipient",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Update a recipient.
*/
        pub async fn update_recipient(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRecipientRequest>,
        ) -> std::result::Result<tonic::Response<super::RecipientInfo>, tonic::Status> {
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
                "/delta_sharing.recipients.v1.RecipientsService/UpdateRecipient",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.recipients.v1.RecipientsService",
                        "UpdateRecipient",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /** Delete a recipient.
*/
        pub async fn delete_recipient(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRecipientRequest>,
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
                "/delta_sharing.recipients.v1.RecipientsService/DeleteRecipient",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.recipients.v1.RecipientsService",
                        "DeleteRecipient",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
