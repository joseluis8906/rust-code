pub mod delivery {
        include!(concat!("pb/tonicpb", "/delivery.rs"));
        pub mod storemanager {
                include!(concat!("pb/tonicpb", "/delivery.storemanager.rs"));
        }
}
