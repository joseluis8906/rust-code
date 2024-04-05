#[cfg(feature = "ssr")]
use crate::pb::delivery::storemanager::store_manager_service_client::StoreManagerServiceClient;

#[cfg(feature = "ssr")]
use axum::extract::FromRef;

#[cfg(feature = "ssr")]
use leptos::LeptosOptions;

#[cfg(feature = "ssr")]
use leptos_router::RouteListing;

#[cfg(feature = "ssr")]
use tonic::transport::Channel;

#[cfg(feature = "ssr")]
#[derive(FromRef, Debug, Clone)]

pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub storemanager_grpc: StoreManagerServiceClient<Channel>,
    pub routes: Vec<RouteListing>,
}
