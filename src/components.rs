use leptos::*;
use wasm_bindgen::JsValue;

#[component]
pub fn Desktop(children: Children) -> impl IntoView {
    view! {
        <div class="conainer mx-auto p-4 h-screen flex items-center justify-center desktop">
            {children()}
        </div>
    }
}

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="card bg-neutral-700 shadow-md shadow-gray-800 mx-auto rounded-lg grid gap-2 grid-cols-1 p-3">
            {children()}
        </div>
    }
}

#[component]
pub fn Text<T, U>(
    #[prop(default = "Input Text")] label: &'static str,
    input: T,
    value: ReadSignal<U>,
) -> impl IntoView
where
    T: FnMut(leptos::ev::Event) + 'static,
    U: std::fmt::Display + 'static + Clone,
    JsValue: From<U>,
{
    view! {
        <input
            class="bg-neutral-800 text-neutral-200 placeholder-neutral-500 rounded-lg shadow-inner shadow-neutral-900 focus:outline focus:outline-2 focus:outline-cyan-600/50 p-2"
            type="text"
            placeholder=label
            on:input=input
            prop:value=value />
    }
}

const DEFAULT_SRC: &str = "https://img.freepik.com/premium-vector/default-image-icon-vector-missing-picture-page-website-design-mobile-app-no-photo-available_87543-11093.jpg?w=1060";
const DEFAULT_ALT: &str = "default image";

#[component]
pub fn Img(
    #[prop(default=DEFAULT_SRC)] src: &'static str,
    #[prop(default=DEFAULT_ALT)] alt: &'static str,
) -> impl IntoView {
    view! {
        <img
            class="rounded-lg shadow-inner shadow-neutral-900"
            src=src
            alt=alt />
    }
}

#[component]
pub fn SelectMenu<T>(
    // #[prop(optional)] searchable: bool,
    #[prop(default = "Select menu")] label: &'static str,
    #[prop(default = vec![])] options: Vec<&'static str>,
    change: T,
    value: ReadSignal<String>,
) -> impl IntoView
where
    T: FnMut(leptos::ev::Event) + 'static,
{
    let option_elms = options
        .into_iter()
        .map(|op| {
            view! {
                <option class="text-neutral-200" value=op selected=move || value() == op>
                    {op}
                </option>
            }
        })
        .collect_view();

    view! {
        <select
            class="bg-neutral-800 text-neutral-200 rounded-lg shadow-inner shadow-neutral-900 focus:outline focus:outline-2 focus:outline-cyan-600/50 p-2"
            class=("text-neutral-500", move || value().is_empty())
            on:change=change
        >
            <option value="" class="text-gray-400">{label}</option>

            {option_elms}
        </select>
    }
}

#[component]
pub fn Label(children: Children) -> impl IntoView {
    view! {
        <label class="text-neutral-200">{children()}</label>
    }
}

#[component]
pub fn Button<T>(label: &'static str, click: T) -> impl IntoView
where
    T: FnMut(leptos::ev::MouseEvent) + 'static,
{
    view! {
        <button
            class="bg-cyan-600/50 text-neutral-200 rounded-lg shadow-md shadow-neutral-900 hover:bg-cyan-600/75 active:bg-cyan-600/50 p-2"
            on:click=click
        >
            {label}
        </button>
    }
}
