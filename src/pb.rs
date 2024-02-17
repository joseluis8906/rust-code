pub mod delivery {
    #[cfg(feature = "ssr")]
    tonic::include_proto!("delivery");
    pub mod storemanager {
        #[cfg(feature = "ssr")]
        tonic::include_proto!("delivery.storemanager");
    }
}
