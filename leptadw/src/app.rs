use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{
        Button, CityActionRow, Desktop, Dialog, EntryRow, HeaderBar, Label, ListBox, ListBoxRow,
        NavigationPage, NavigationSplitView, NavigationView, Product, SearchEntry, StoreActionRow,
        Window, WindowAction, WindowContent, WindowTitle,
};

use crate::types;

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::pb::delivery::storemanager::AddStoreRequest;
        use crate::pb::delivery::{Store, Product, Money};

        use crate::pb::delivery::storemanager::store_manager_service_client::StoreManagerServiceClient;
        use tonic::transport::Channel;

        pub fn client() -> Result<StoreManagerServiceClient<Channel>, ServerFnError> {
            use_context::<StoreManagerServiceClient<Channel>>()
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
            <Stylesheet id="leptos" href="/pkg/leptadw.css"/>

            // sets the document title
            <Title text="Welcome to Leptos"/>

            // content for this welcome page
            <Router fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! { <ErrorTemplate outside_errors/> }.into_view()
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
        view! { <TestNavigationSplitView/> }
}

#[server(AddStore)]
pub async fn add_a_store(store: Store) -> Result<(), ServerFnError> {
        let mut client = client()?;

        let req = AddStoreRequest { store: Some(store) };

        let mut request = tonic::Request::new(req);

        request.metadata_mut()
                .insert("x-auth-email", "john.doe@example.com".parse().unwrap());

        let response = client.add_store(request).await;
        match response {
                Ok(_) => Ok(()),
                Err(e) => Err(ServerFnError::ServerError(e.to_string())),
        }
}

#[component]
pub fn AddStore() -> impl IntoView {
        let (name, set_name) = create_signal(String::from(""));
        let (city, set_city) = create_signal(types::CityCountry {
                city: "",
                country: "",
        });
        let (address, set_address) = create_signal(String::from(""));
        let (show_dialog, set_show_dialog) = create_signal(false);

        let reset = move || {
                set_name(String::from(""));
                set_city(types::CityCountry {
                        city: "",
                        country: "",
                });
                set_address(String::from(""));
        };

        view! {
            <Window>
                <WindowTitle>Register a Store</WindowTitle>
                <WindowContent>
                    <EntryRow
                        label=String::from("Name")
                        value=name
                        on_input=move |ev| set_name(event_target_value(&ev))
                    />

                    <Dialog show=show_dialog>
                        <HeaderBar class="bg-osd".to_string() title=String::from("Select a City")>
                            <SearchEntry/>
                        </HeaderBar>
                        <ListBox>
                            <ListBoxRow on_click=move |_| {
                                set_city(types::CityCountry {
                                    city: "Bucaramanga",
                                    country: "Colombia",
                                });
                                set_show_dialog(false);
                            }>

                                <div class="flex items-center">
                                    <div class="leading-tight">
                                        <span class="block">Bucaramanga, <b>Colombia</b></span>
                                        <span class="text-xs text-neutral-300">
                                            America/Bogota {"·"} UTC-0500
                                        </span>
                                    </div>
                                    <div class="grow text-sm text-right text-neutral-300">
                                        <span>Fri 16:56</span>
                                    </div>
                                </div>
                            </ListBoxRow>
                            <ListBoxRow on_click=move |_| {
                                set_city(types::CityCountry {
                                    city: "Buenos Aires",
                                    country: "Argentina",
                                });
                                set_show_dialog(false);
                            }>

                                <div class="flex items-center">
                                    <div class="leading-tight">
                                        <span class="block">Buenos Aires, <b>Argentina</b></span>
                                        <span class="text-xs text-neutral-300">
                                            America/Buenos Aires {"·"} UTC-0400
                                        </span>
                                    </div>
                                    <div class="grow text-sm text-right text-neutral-300">
                                        <span>Fri 15:56</span>
                                    </div>
                                </div>
                            </ListBoxRow>
                        </ListBox>
                    </Dialog>

                    <CityActionRow
                        title=String::from("City")
                        subtitle=String::from("Chose a city from list...")
                        value=city
                        on_click=move |_| {
                            set_show_dialog(true);
                        }
                    />

                    <EntryRow
                        label=String::from("Address")
                        value=address
                        on_input=move |ev| set_address(event_target_value(&ev))
                    />

                    <div class="flex justify-between">
                        <Button label=String::from("Cancel") on_click=move |_| reset()/>

                        <Button
                            label=String::from("Save")
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
                                    let res = add_a_store(store).await;
                                    match res {
                                        Ok(_) => {
                                            logging::log!("store registered successfully!");
                                            reset()
                                        }
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
pub fn AddProduct() -> impl IntoView {
        let initialStore = Store {
                name: Some(String::from("Burger King Cabecera")),
                country: Some(String::from("Colombia")),
                city: Some(String::from("Bucaramanga")),
                address: Some(String::from("Cra 27 # 54 - 12")),
                products: vec![],
        };

        let initialProduct = Product {
                r#ref: Some(String::from("BK-001")),
                name: Some(String::from("Whopper")),
                price: Some(Money {
                        currency: Some(String::from("COP")),
                        amount: Some(10000),
                }),
        };

        let (store, _set_store) = create_signal(initialStore);
        let (product, _set_product) = create_signal(initialProduct);

        view! {
            <Window>
                <WindowTitle>Register a Product</WindowTitle>
                <WindowContent>
                    <StoreActionRow value=store on_click=move |_| { logging::log!("it works") }/>
                    <Product value=product on_change=move |_| {}/>
                    <WindowAction
                        accept_label=String::from("Save")
                        on_accept=move |_| {
                            logging::log!("product registered successfully!");
                        }

                        cancel_label=String::from("Cancel")
                        on_cancel=move |_| {
                            logging::log!("product registration canceled!");
                        }
                    />

                </WindowContent>
            </Window>
        }
}

#[component]
pub fn TestNavigationView() -> impl IntoView {
        let (page, set_page) = create_signal(1);

        view! {
            <Window>
                <NavigationView>
                    <NavigationPage
                        title=String::from("Page 1")
                        page=1
                        current_page=Box::new(page)
                        on_back=Box::new(move || {})
                    >
                        <WindowContent>
                            <Button
                                label=String::from("Open Page 2")
                                on_click=move |_| { set_page(2) }
                            />
                        </WindowContent>
                    </NavigationPage>
                    <NavigationPage
                        title=String::from("Page 2")
                        page=2
                        current_page=Box::new(page)
                        prev=true
                        on_back=Box::new(move || {
                            set_page(1);
                        })
                    >

                        <WindowContent>
                            <Button
                                label=String::from("Open Page 3")
                                on_click=move |_| { set_page(3) }
                            />
                        </WindowContent>
                    </NavigationPage>
                    <NavigationPage
                        title=String::from("Page 3")
                        page=3
                        current_page=Box::new(page)
                        prev=true
                        on_back=Box::new(move || {
                            set_page(2);
                        })
                    >

                        <WindowContent>
                            <Button label=String::from("Page 3") on_click=move |_| {}/>
                        </WindowContent>
                    </NavigationPage>
                </NavigationView>
            </Window>
        }
}

#[component]
pub fn TestNavigationSplitView() -> impl IntoView {
        view! {
            <Window>
                <NavigationSplitView class=String::from("h-full")>
                    <NavigationPage class=String::from("flex-none block w-64 h-full")>
                        <Label>Side</Label>
                    </NavigationPage>

                    <NavigationPage class=String::from("w-full")>
                        <WindowContent>
                            <Label>Content</Label>
                        </WindowContent>
                    </NavigationPage>
                </NavigationSplitView>
            </Window>
        }
}
