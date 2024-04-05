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
