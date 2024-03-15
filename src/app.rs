use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{
    Button, CityActionRow, Desktop, Dialog, EntryRow, HeaderBar, ListBox, ListBoxRow, Product,
    SearchEntry, StoreActionRow, Window, WindowAction, WindowContent, WindowTitle,
};

use crate::types;

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::pb::delivery::storemanager::RegistersAStoreRequest;
        use crate::pb::delivery::{Store, Product, Money};

        use crate::pb::delivery::storemanager::store_manager_client::StoreManagerClient;
        use tonic::transport::Channel;

        pub fn client() -> Result<StoreManagerClient<Channel>, ServerFnError> {
            use_context::<StoreManagerClient<Channel>>()
                .ok_or_else(|| ServerFnError::ServerError("gRPC StoreManagerClient missing.".into()))
        }
    }else{
        use crate::pb::delivery::{Store, Product, Money};
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/playground.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Desktop>
                    <Routes>
                        <Route path="/storemanager" view=StoreManagerPage/>
                    </Routes>
                </Desktop>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn StoreManagerPage() -> impl IntoView {
    view! {
        <RegistersAStore />
        // <RegistersProducts />
    }
}

#[server(RegistersAStore)]
pub async fn registers_a_store(store: Store) -> Result<(), ServerFnError> {
    let mut client = client()?;

    let req = RegistersAStoreRequest { store: Some(store) };

    let mut request = tonic::Request::new(req);

    request
        .metadata_mut()
        .insert("x-auth-email", "john.doe@example.com".parse().unwrap());

    let response = client.registers_a_store(request).await;
    match response {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[component]
pub fn RegistersAStore() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());
    let (city, set_city) = create_signal(types::CityCountry {
        city: "",
        country: "",
    });
    let (address, set_address) = create_signal("".to_string());
    let (show_dialog, set_show_dialog) = create_signal(false);

    let reset = move || {
        set_name("".to_string());
        set_city(types::CityCountry {
            city: "",
            country: "",
        });
        set_address("".to_string());
    };

    view! {
        <Window>
            <WindowTitle>Register a Store</WindowTitle>
            <WindowContent>
                <EntryRow
                    label="Name".to_string()
                    value=name
                    on_input=move |ev| set_name(event_target_value(&ev))
                />

                <Dialog show=show_dialog>
                    <HeaderBar class="bg-osd".to_string() title="Select a City".to_string()>
                        <SearchEntry />
                    </HeaderBar>
                    <ListBox>
                        <ListBoxRow
                            on_click=move |_| {
                                set_city(types::CityCountry {
                                    city: "Bucaramanga",
                                    country: "Colombia",
                                });
                                set_show_dialog(false);
                            }
                        >
                            <div class="flex items-center">
                                <div class="leading-tight">
                                    <span class="block">Bucaramanga, <b>Colombia</b></span>
                                    <span class="text-xs text-neutral-300">America/Bogota {"·"} UTC-0500</span>
                                </div>
                                <div class="grow text-sm text-right text-neutral-300">
                                    <span>Fri 16:56</span>
                                </div>
                            </div>
                        </ListBoxRow>
                        <ListBoxRow
                            on_click=move |_| {
                                set_city(types::CityCountry {
                                    city: "Buenos Aires",
                                    country: "Argentina",
                                });
                                set_show_dialog(false);
                            }
                        >
                            <div class="flex items-center">
                                <div class="leading-tight">
                                    <span class="block">Buenos Aires, <b>Argentina</b></span>
                                    <span class="text-xs text-neutral-300">America/Buenos Aires {"·"} UTC-0400</span>
                                </div>
                                <div class="grow text-sm text-right text-neutral-300">
                                    <span>Fri 15:56</span>
                                </div>
                            </div>
                        </ListBoxRow>
                    </ListBox>
                </Dialog>

                <CityActionRow
                    title="City".to_string()
                    subtitle="Chose a city from list...".to_string()
                    value=city
                    on_click=move |_| {
                        set_show_dialog(true);
                    }
                />

                <EntryRow
                    label="Address".to_string()
                    value=address
                    on_input=move |ev| set_address(event_target_value(&ev))
                />

                <div class="flex justify-between">
                    <Button
                        label="Cancel".to_string()
                        on_click=move |_| reset()
                />

                    <Button
                        label="Save".to_string()
                        primary=true
                        on_click=move |_| {
                            let city = city();

                            let store = Store {
                                name: Some(name().to_string()),
                                country: Some(city.country.to_string()),
                                city: Some(city.city.to_string()),
                                address: Some(address().to_string()),
                                products: vec![],
                            };

                            spawn_local(async move {
                                let res = registers_a_store(store).await;
                                match res {
                                    Ok(_) => {
                                            logging::log!("store registered successfully!");
                                            reset()
                                        },
                                    Err(e) => logging::error!("{:?}", e),
                                }
                            });
                        }
                    />
                </div>

            </WindowContent>

        </Window>
    }
}

#[component]
pub fn RegistersProducts() -> impl IntoView {
    let initialStore = Store {
        name: Some("Burger King Cabecera".to_string()),
        country: Some("Colombia".to_string()),
        city: Some("Bucaramanga".to_string()),
        address: Some("Cra 27 # 54 - 12".to_string()),
        products: vec![],
    };

    let initialProduct = Product {
        r#ref: Some("BK-001".to_string()),
        name: Some("Whopper".to_string()),
        price: Some(Money {
            currency: Some("COP".to_string()),
            amount: Some(10000),
        }),
    };

    let (store, _set_store) = create_signal(initialStore);
    let (product, _set_product) = create_signal(initialProduct);

    view! {
        <Window>
            <WindowTitle>Register a Product</WindowTitle>
            <WindowContent>
                <StoreActionRow value=store on_click=move |_| { logging::log!("it works") } />
                <Product value=product on_change=move |_| {} />
                <WindowAction
                    accept_label="Save".to_string()
                    on_accept=move |_| {
                        logging::log!("product registered successfully!");
                    }
                    cancel_label="Cancel".to_string()
                    on_cancel=move |_| {
                        logging::log!("product registration canceled!");
                    }
                />
            </WindowContent>
        </Window>
    }
}
