use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/{{project-name}}.css"/>

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
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <BusyButton />
    }
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use leptos::*;

    use crate::pb::delivery::storemanager::store_manager_client::StoreManagerClient;
    use tonic::transport::Channel;

    pub fn client() -> Result<StoreManagerClient<Channel>, ServerFnError> {
        use_context::<StoreManagerClient<Channel>>()
            .ok_or_else(|| ServerFnError::ServerError("gRPC StoreManagerClient missing.".into()))
    }
}

#[server(RegistersAStore)]
pub async fn registers_a_store() -> Result<(), ServerFnError> {
    use self::ssr::*;
    use crate::pb::delivery::storemanager::RegistersAStoreRequest;
    use crate::pb::delivery::Store;

    let mut client = client()?;

    let mut request = tonic::Request::new(RegistersAStoreRequest {
        store: Some(Store {
            name: Some("Burger King Cabecera".to_string()),
            country: Some("Colombia".to_string()),
            city: Some("Bucaramanga".to_string()),
            address: Some("Cra 33 # 22 - 18".to_string()),
            products: vec![],
        }),
    });

    request
        .metadata_mut()
        .insert("x-auth-email", "john.doe@example.com".parse().unwrap());

    let response = client.registers_a_store(request).await.unwrap();

    println!("RESPONSE={:?}", response);

    Ok(())
}

#[component]
pub fn BusyButton() -> impl IntoView {
    view! {
        <button on:click=move |_| {
            spawn_local(async {
                let _ = registers_a_store().await;
            });
        }>
            "Registers A Store"
        </button>
    }
}
