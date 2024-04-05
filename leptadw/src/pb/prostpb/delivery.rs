#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Id {
    #[prost(message, optional, tag = "1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub number: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Money {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "2")]
    pub currency: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Customer {
    #[prost(message, optional, tag = "1")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub city: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Courier {
    #[prost(message, optional, tag = "5")]
    pub id: ::core::option::Option<Id>,
    #[prost(message, optional, tag = "1")]
    pub email: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub city: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Store {
    #[prost(message, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub country: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub city: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "5")]
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub products: ::prost::alloc::vec::Vec<Product>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Product {
    #[prost(message, optional, tag = "1")]
    pub r#ref: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<Money>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    #[prost(message, optional, tag = "1")]
    pub code: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub customer: ::core::option::Option<Customer>,
    #[prost(message, optional, tag = "4")]
    pub store: ::core::option::Option<Store>,
}
