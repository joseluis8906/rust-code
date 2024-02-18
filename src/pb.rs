use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub mod delivery {
            include!(concat!("pb/tonicpb", "/delivery.rs"));
            pub mod storemanager {
                include!(concat!("pb/tonicpb", "/delivery.storemanager.rs"));
            }
        }
    }else{
        pub mod delivery {
            include!(concat!("pb/prostpb", "/delivery.rs"));
            pub mod storemanager {
                include!(concat!("pb/prostpb", "/delivery.storemanager.rs"));
            }
        }
    }
}
