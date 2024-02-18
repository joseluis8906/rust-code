use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{Button, Card, Desktop, Img, SelectMenu, Text};
use crate::forms;

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
pub fn ProductForm() -> impl IntoView {
    let (name, set_name) = create_signal("".to_string());
    let (country, set_country) = create_signal("".to_string());
    let (city, set_city) = create_signal("".to_string());
    let (address, set_address) = create_signal("".to_string());

    view! {
        <Card>
            <Img
                src="https://img.freepik.com/free-vector/shop-with-sign-we-are-open_52683-38687.jpg?w=1380&t=st=1708213595~exp=1708214195~hmac=25f7a3f447093dff2aaa89ecf6237e3a659312d2204606ca597213c0c0271fb8"
                alt="store"/>

            <Text
                label="Name"
                input=move |ev| set_name(event_target_value(&ev))
                value=name />

            <SelectMenu
                // searchable
                label="Search a country..."
                options=vec!["Argentina", "Colombia", "United States"]
                value=country
                change=move |ev| {
                    let new_value = event_target_value(&ev);
                    set_country(new_value);
                }/>

            <SelectMenu
                // searchable
                label="Search a city..."
                options=vec!["Bogota", "Medellin", "Bucaramanga"]
                value=city
                change=move |ev| {
                    let new_value = event_target_value(&ev);
                    set_city(new_value);
                }/>

            <Text
                label="Address"
                input=move |ev| set_address(event_target_value(&ev))
                value=address />

            <Button
                label="Registers A Store"
                click=move |_| {
                    let store = forms::Store {
                        name: Some(name().to_string()),
                        country: Some(country().to_string()),
                        city: Some(city().to_string()),
                        address: Some(address().to_string()),
                    };

                    spawn_local(async move {
                        let _ = registers_a_store(store).await;
                    });
                }
            />
        </Card>
    }
}
