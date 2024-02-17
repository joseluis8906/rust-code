use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <BusyButton />
    }
}

pub mod forms {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Store {
        pub name: Option<String>,
        pub country: Option<String>,
        pub city: Option<String>,
        pub address: Option<String>,
    }
}

#[server(RegistersAStore)]
pub async fn registers_a_store(form: forms::Store) -> Result<(), ServerFnError> {
    use self::ssr::*;
    use crate::pb::delivery::storemanager::RegistersAStoreRequest;
    use crate::pb::delivery::Store;

    let mut client = client()?;

    let mut request = tonic::Request::new(RegistersAStoreRequest {
        store: Some(Store {
            name: form.name,
            country: form.country,
            city: form.city,
            address: form.address,
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
                let store = forms::Store {
                    name: Some("Burger King Cabecera".to_string()),
                    country: Some("Colombia".to_string()),
                    city: Some("Bucaramanga".to_string()),
                    address: Some("Cra 33 # 22 - 18".to_string()),
                };
                let _ = registers_a_store(store).await;
            });
        }>
            "Registers A Store"
        </button>
    }
}
