#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddStoreRequest {
    #[prost(message, optional, tag = "1")]
    pub store: ::core::option::Option<super::Store>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddProductRequest {
    #[prost(message, optional, tag = "1")]
    pub store: ::core::option::Option<super::Store>,
    #[prost(message, optional, tag = "2")]
    pub product: ::core::option::Option<super::Product>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderReceivedRequest {
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<super::Order>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderIsReadyRequest {
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<super::Order>,
}
/// Generated client implementations.
pub mod store_manager_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct StoreManagerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StoreManagerServiceClient<tonic::transport::Channel> {
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
    impl<T> StoreManagerServiceClient<T>
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
        ) -> StoreManagerServiceClient<InterceptedService<T, F>>
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
            StoreManagerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn add_store(
            &mut self,
            request: impl tonic::IntoRequest<super::AddStoreRequest>,
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
                "/delivery.storemanager.StoreManagerService/AddStore",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delivery.storemanager.StoreManagerService",
                        "AddStore",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_product(
            &mut self,
            request: impl tonic::IntoRequest<super::AddProductRequest>,
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
                "/delivery.storemanager.StoreManagerService/AddProduct",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delivery.storemanager.StoreManagerService",
                        "AddProduct",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn order_received(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderReceivedRequest>,
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
                "/delivery.storemanager.StoreManagerService/OrderReceived",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delivery.storemanager.StoreManagerService",
                        "OrderReceived",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn order_is_ready(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderIsReadyRequest>,
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
                "/delivery.storemanager.StoreManagerService/OrderIsReady",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "delivery.storemanager.StoreManagerService",
                        "OrderIsReady",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
