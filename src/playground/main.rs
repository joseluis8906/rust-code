use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| view! { <App/> });
}

#[component]
fn App() -> impl IntoView {
    let (data, _) = create_signal(vec![
        DBEntry {
            key: "foo".to_string(),
            value: create_rw_signal(10),
        },
        DBEntry {
            key: "bar".to_string(),
            value: create_rw_signal(20),
        },
        DBEntry {
            key: "baz".to_string(),
            value: create_rw_signal(15),
        },
    ]);

    view! {
        <button on:click=move |_| {
            data.with(|data| {
                for row in data {
                    row.value.update(|value| *value *= 2);
                }
            });
        }> "Multiply" </button>

        <For
            each=data
            key=|state| state.key.clone()
            let:child>

            <p>{child.value}</p>
        </For>
    }
}

#[derive(Debug, Clone)]
struct DBEntry {
    key: String,
    value: RwSignal<i32>,
}
