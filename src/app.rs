use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{
    ActionRow, Button, Desktop, Dialog, EntryRow, HeaderBar, ListBox, ListBoxRow, SearchEntry,
    Window,
};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::pb::delivery::storemanager::RegistersAStoreRequest;
        use crate::pb::delivery::Store;

        use crate::pb::delivery::storemanager::store_manager_client::StoreManagerClient;
        use tonic::transport::Channel;

        pub fn client() -> Result<StoreManagerClient<Channel>, ServerFnError> {
            use_context::<StoreManagerClient<Channel>>()
                .ok_or_else(|| ServerFnError::ServerError("gRPC StoreManagerClient missing.".into()))
        }
    }else{
        use crate::pb::delivery::Store;
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
                        <Route path="" view=HomePage/>
                    </Routes>
                </Desktop>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <ProductForm />
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

    let response = client.registers_a_store(request).await.unwrap();

    println!("RESPONSE={:?}", response);

    Ok(())
}

#[component]
pub fn ProductForm() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());
    let (country, _set_country) = create_signal("".to_string());
    let (city, _set_city) = create_signal("".to_string());
    let (address, set_address) = create_signal("".to_string());

    view! {
        <Window class="relative">
            // <Img
            //     src="https://img.freepik.com/free-vector/shop-with-sign-we-are-open_52683-38687.jpg?w=1380&t=st=1708213595~exp=1708214195~hmac=25f7a3f447093dff2aaa89ecf6237e3a659312d2204606ca597213c0c0271fb8"
            //     alt="store"/>

            <EntryRow
                label="Name"
                value=name
                on_input=set_name />

            <Dialog>
                <HeaderBar>Select A City</HeaderBar>
                <ListBox>
                    <ListBoxRow title="Row One"/>
                    <ListBoxRow title="Row Two"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                    <ListBoxRow title="Row Three"/>
                </ListBox>
            </Dialog>

            <SearchEntry />

            <ActionRow
                title="Country"
                // subtitle="Chose a country from list..."
            />

            <EntryRow
                label="Address"
                value=address
                on_input=set_address />

            <Button
                label="Registers A Store"
                on_click=move |_| {
                    let store = Store {
                        name: Some(name().to_string()),
                        country: Some(country().to_string()),
                        city: Some(city().to_string()),
                        address: Some(address().to_string()),
                        products: vec![],
                    };

                    spawn_local(async move {
                        let res = registers_a_store(store).await;
                        match res {
                            Ok(_) => logging::log!("store registered successfully!"),
                            Err(e) => logging::error!("registering store: {:?}", e),
                        }
                    });
                }
            />
        </Window>
    }
}
