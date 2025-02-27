// @generated
/// Generated client implementations.
pub mod credentials_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /** Manage credentials to access external data sources and services
 as well as generate signed urls for the Delta Sharing service.
*/
    #[derive(Debug, Clone)]
    pub struct CredentialsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CredentialsServiceClient<tonic::transport::Channel> {
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
    impl<T> CredentialsServiceClient<T>
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
        ) -> CredentialsServiceClient<InterceptedService<T, F>>
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
            CredentialsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        ///
        pub async fn list_credentials(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCredentialsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListCredentialsResponse>,
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
                "/delta_sharing.credentials.v1.CredentialsService/ListCredentials",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.credentials.v1.CredentialsService",
                        "ListCredentials",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn create_credential(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCredentialRequest>,
        ) -> std::result::Result<tonic::Response<super::CredentialInfo>, tonic::Status> {
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
                "/delta_sharing.credentials.v1.CredentialsService/CreateCredential",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.credentials.v1.CredentialsService",
                        "CreateCredential",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn get_credential(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCredentialRequest>,
        ) -> std::result::Result<tonic::Response<super::CredentialInfo>, tonic::Status> {
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
                "/delta_sharing.credentials.v1.CredentialsService/GetCredential",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.credentials.v1.CredentialsService",
                        "GetCredential",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn update_credential(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCredentialRequest>,
        ) -> std::result::Result<tonic::Response<super::CredentialInfo>, tonic::Status> {
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
                "/delta_sharing.credentials.v1.CredentialsService/UpdateCredential",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.credentials.v1.CredentialsService",
                        "UpdateCredential",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn delete_credential(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCredentialRequest>,
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
                "/delta_sharing.credentials.v1.CredentialsService/DeleteCredential",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delta_sharing.credentials.v1.CredentialsService",
                        "DeleteCredential",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
