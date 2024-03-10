use bson::oid::ObjectId;
use leptos::*;
use leptos_use::{use_intl_number_format, NumberStyle, UseIntlNumberFormatOptions};

use crate::pb::delivery::{Money, Product, Store};
use crate::types;

#[component]
pub fn Desktop(children: Children) -> impl IntoView {
    view! {
        <div class="conainer mx-auto p-1 sm:p-2 md:p-4 w-screen h-screen flex items-center justify-center desktop">
            {children()}
        </div>
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
            class="rounded-lg"
            src=src
            alt=alt />
    }
}

#[component]
pub fn Label(children: Children) -> impl IntoView {
    view! {
        <label class="text-neutral-200">{children()}</label>
    }
}

#[component]
pub fn Button<T>(
    label: String,
    on_click: T,
    #[prop(default = false)] primary: bool,
) -> impl IntoView
where
    T: FnMut(leptos::ev::MouseEvent) + 'static,
{
    view! {
        <button
            class="min-w-24 w-fit text-neutral-200 shadow-none rounded-lg shadow-md shadow-neutral-900 py-2 px-6"
            class=("bg-primary-500", move || primary)
            class=("hover:bg-primary-550", move || primary)
            class=("active:bg-primary-600", move || primary)
            class=("bg-foreground", move || !primary)
            class=("hover:bg-hover", move || !primary)
            class=("active:bg-active", move || !primary)
            on:click=on_click
        >
            {label}
        </button>
    }
}

#[component]
pub fn SelectMenu(
    #[prop(optional)] label: &'static str,
    options: Vec<&'static str>,
    on_change: WriteSignal<String>,
    value: ReadSignal<String>,
) -> impl IntoView {
    let (hidden, set_hidden) = create_signal(true);

    view! {
        <div class="relative">
            <button
                class="relative w-full cursor-default rounded-lg bg-neutral-800 text-neutral-200 py-1.5 pl-3 pr-10 text-left shadow-inner shadow-neutral-900 focus:outline focus:outline-2 focus:outline-cyan-600/50"
                on:click=move|_| set_hidden(!hidden())
            >
                <span class="flex items-center">
                    <span
                        class="block truncate"
                        class=("text-neutral-500", move || {value().to_string().is_empty()})
                    >
                    {
                        move|| {
                            if value().to_string().is_empty() {
                                label.to_string()
                            } else {
                                value().to_string()
                            }
                        }
                    }
                    </span>
                </span>
                <span class="pointer-events-none absolute inset-y-0 right-0 ml-3 flex items-center pr-2">
                    <svg class="h-5 w-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor">
                        <path fill-rule="evenodd" d="M10 3a.75.75 0 01.55.24l3.25 3.5a.75.75 0 11-1.1 1.02L10 4.852 7.3 7.76a.75.75 0 01-1.1-1.02l3.25-3.5A.75.75 0 0110 3zm-3.76 9.2a.75.75 0 011.06.04l2.7 2.908 2.7-2.908a.75.75 0 111.1 1.02l-3.25 3.5a.75.75 0 01-1.1 0l-3.25-3.5a.75.75 0 01.04-1.06z" clip-rule="evenodd" />
                    </svg>
                </span>
            </button>

            <ul
                class="absolute z-10 mt-1 max-h-56 w-full overflow-auto rounded-lg bg-neutral-800 py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none"
                class:hidden=hidden
                tabindex="-1"
                role="listbox"
            >
                <li
                    class="text-neutral-200 relative cursor-default select-none py-2 pl-3 pr-9 hover:bg-neutral-600"
                    id="listbox-option-0"
                    role="option"
                    on:click=move |_| {
                        set_hidden(true);
                        on_change("".to_string())
                    }

                >
                    <div class="flex items-center">
                        <span class="text-neutral-500 font-normal ml-3 block truncate">Select a/an {label}...</span>
                    </div>

                    <span class="hidden text-cyan-600/75 absolute inset-y-0 right-0 flex items-center pr-4">
                        <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                        </svg>
                    </span>
                </li>
                {
                    options.into_iter().enumerate().map(move |(id, op)| {
                        view!{
                            <li
                                class="text-neutral-200 relative cursor-default select-none py-2 pl-3 pr-9 hover:bg-neutral-600"
                                id={format!("listbox-option-{}", id + 1)}
                                role="option"
                                on:click=move |_| {
                                    set_hidden(true);
                                    on_change(op.to_string())
                                }

                            >
                                <div class="flex items-center">
                                    // <img src="https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80" alt="" class="h-5 w-5 flex-shrink-0 rounded-full" />
                                    <span class="font-normal ml-3 block truncate">{op}</span>
                                </div>

                                <span
                                    class="text-cyan-600/75 absolute inset-y-0 right-0 flex items-center pr-4"
                                    class:hidden=move || value() != op
                                >
                                    <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                        <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                                    </svg>
                                </span>
                            </li>
                        }
                    }).collect_view()
                }
            </ul>
        </div>
    }
}

#[component]
pub fn StatusPage() -> impl IntoView {
    view! {}
}

#[component]
pub fn ToastOverlay() -> impl IntoView {
    view! {}
}

#[component]
pub fn Banner() -> impl IntoView {
    view! {}
}

#[component]
pub fn Avatar() -> impl IntoView {
    view! {}
}

#[component]
pub fn Dialog(show: ReadSignal<bool>, children: Children) -> impl IntoView {
    view! {
        <div
            class="absolute w-full h-full bg-osd/50 top-0 left-0 z-50 rounded-lg flex items-center justify-center"
            class:hidden=move || !show()
        >
            <div class="min-w-80 min-h-80 h-min bg-osd shadow-lg shadow-neutral-900 mx-auto rounded-lg border-zinc-50/10 border">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn AlertDialog() -> impl IntoView {
    view! {}
}

#[component]
pub fn AboutDialog() -> impl IntoView {
    view! {}
}

#[component]
pub fn CityActionRow<T: FnMut(leptos::ev::MouseEvent) + 'static>(
    on_click: T,
    value: ReadSignal<types::CityCountry>,
    #[prop(optional)] title: String,
    #[prop(optional)] subtitle: String,
    #[prop(default = "angle-right".to_string())] icon: String,
) -> impl IntoView {
    let (title, _) = create_signal(title);
    let (subtitle, _) = create_signal(subtitle);
    let (city, set_city) = create_signal("".to_string());
    let (country, set_country) = create_signal("".to_string());

    create_effect(move |_| {
        let city = value().city.to_string();
        let country = value().country.to_string();

        set_city(city);
        set_country(country);
    });

    let is_city_not_empty = move || !city().is_empty() && !country().is_empty();
    let is_city_empty = move || city().is_empty() || country().is_empty();
    let is_subtitle_not_empty = move || !subtitle().is_empty();
    let is_subtitle_empty = move || subtitle().is_empty();
    let is_subtitle_empty_and_city_not =
        move || subtitle().is_empty() || (!city().is_empty() && !country().is_empty());

    view! {
        <div
            class="flex items-center relative bg-foreground text-neutral-200 placeholder-neutral-500 rounded-lg \
            hover:bg-hover active:bg-foreground focus:outline-none focus:ring-2 focus:ring-blue-300/50 px-3 py-0.5 w-full \
            appearance-none"
            on:click=on_click
        >
            <div class="mr-3">
                <label
                    class="text-md"
                    class=("text-neutral-200", is_city_not_empty)
                    class=("text-neutral-400", is_city_empty)
                    class=("block", is_subtitle_not_empty)
                    class=("inline-block", is_subtitle_empty)
                    class=("py-3", is_subtitle_empty_and_city_not)
                >
                    {title}
                </label>
                <span
                    class="text-xs text-neutral-400"
                    class:hidden=is_subtitle_empty_and_city_not
                >
                    {subtitle}
                </span>
            </div>
            <div
                class="grow text-right pr-4"
                class:hidden=move || city().is_empty() || country().is_empty()
            >
                <span>{city}</span>, <strong>{country}</strong>
            </div>
            <button
                type="button"
                class="absolute inset-y-0 end-2 bg-transparent focus:outline-none p-0"
            >
                <Icon name=icon />
            </button>
        </div>
    }
}

#[component]
pub fn SwitchRow() -> impl IntoView {
    view! {}
}

#[component]
pub fn ComboRow() -> impl IntoView {
    view! {}
}

#[component]
pub fn ExpanderRow() -> impl IntoView {
    view! {}
}

#[component]
pub fn EntryRow<T: FnMut(leptos::ev::Event) + 'static>(
    #[prop(default = "Input Text".to_string())] label: String,
    on_input: T,
    value: ReadSignal<String>,
) -> impl IntoView {
    let (hidden, set_hidden) = create_signal(false);
    let id = format!("floating-filled-{}", ObjectId::new());

    view! {
        <div class="relative">
            <input
                id=id.to_owned()
                class="block bg-foreground text-neutral-200 placeholder-neutral-500 rounded-lg hover:bg-hover \
                        active:bg-foreground focus:outline-none focus:ring-2 focus:ring-blue-300/50 pt-5 pb-2.5 px-3 \
                        w-full appearance-none peer"
                type="text"
                placeholder=" "
                on:input=on_input
                on:focus=move |_| set_hidden(true)
                on:blur=move |_| set_hidden(false)
                prop:value=value
            />

            <label for=id class="absolute text-md text-neutral-400 duration-300 transform -translate-y-4 \
                    scale-75 top-4 z-10 origin-[0] start-2.5 peer-focus:text-neutral-200 peer-placeholder-shown:scale-100 \
                    peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-4 \
                    rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto"
            >
                {label}
            </label>

            <button
                type="button"
                class="absolute end-2 bottom-5 bg-transparent text-neutral-200 focus:outline-none p-0"
                class:hidden=hidden
            >
                <Icon name="edit".to_string() />
            </button>
        </div>
    }
}

#[component]
pub fn PasswordEntryRow() -> impl IntoView {
    view! {}
}

#[component]
pub fn SpinRow() -> impl IntoView {
    view! {}
}

#[component]
pub fn PreferenceGroup() -> impl IntoView {
    view! {}
}

#[component]
pub fn PreferencePage() -> impl IntoView {
    view! {}
}

#[component]
pub fn PreferenceDialog() -> impl IntoView {
    view! {}
}

#[component]
pub fn NavigationView(children: Children) -> impl IntoView {
    view! {
        <div class="">
            {children()}
        </div>
    }
}

#[component]
pub fn NavigationPage(
    #[prop(default = "Page".to_string())] title: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="flex items-center">
            <div class="px-3"><Icon name="angle-left".to_string() /></div>
            <WindowTitle class="w-full".to_string()>{title}</WindowTitle>
        </div>
        <WindowContent>
            {children()}
        </WindowContent>
    }
}

#[component]
pub fn NavigationSplitView() -> impl IntoView {
    view! {}
}

#[component]
pub fn OverlaySplitView() -> impl IntoView {
    view! {}
}

#[component]
pub fn Carousel() -> impl IntoView {
    view! {}
}

#[component]
pub fn ViewSwitcher() -> impl IntoView {
    view! {}
}

#[component]
pub fn ViewSwitcherBar() -> impl IntoView {
    view! {}
}

#[component]
pub fn TabBar() -> impl IntoView {
    view! {}
}

#[component]
pub fn TabOverview() -> impl IntoView {
    view! {}
}

#[component]
pub fn TabButton() -> impl IntoView {
    view! {}
}

#[component]
pub fn Clamp() -> impl IntoView {
    view! {}
}

#[component]
pub fn ToolbarView() -> impl IntoView {
    view! {}
}

#[component]
pub fn WindowTitle(#[prop(optional)] class: String, children: Children) -> impl IntoView {
    view! {
        <div
            class="text-md text-neutral-200 text-center py-3"
            class=class
        >
            <strong>{children()}</strong>
        </div>
    }
}

#[component]
pub fn HeaderBar(
    #[prop(optional)] title: String,
    #[prop(optional)] class: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class="bg-foreground w-full rounded-t-lg flex grid gap-2 p-3"
            class=class
        >
            <strong class="text-neutral-200 text-center">{title}</strong>
            {children()}
        </div>
    }
}

#[component]
pub fn Window(children: Children) -> impl IntoView {
    view! {
        <div
            class="relative min-w-80 w-11/12 sm:w-10/12 md:w-3/5 bg-background shadow-xl shadow-black/50 mx-auto rounded-lg \
                border-zinc-50/10 border p-0"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn WindowContent(children: Children) -> impl IntoView {
    view! {
        <div class="grid gap-5 grid-cols-1 py-4 sm:py-8 lg:py-16 px-4 sm:px-16 lg:px-32 xl:px-48 2xl:px-64">
            {children()}
        </div>
    }
}

#[component]
pub fn SplitButton() -> impl IntoView {
    view! {}
}

#[component]
pub fn ButtonContent() -> impl IntoView {
    view! {}
}

#[component]
pub fn Bin() -> impl IntoView {
    view! {}
}

#[component]
pub fn ListBox(children: Children) -> impl IntoView {
    view! {
        <div class="max-h-64 overflow-auto scroll-smoth">
            {children()}
        </div>
    }
}

#[component]
pub fn ListBoxRow<T: FnMut(leptos::ev::MouseEvent) + 'static>(
    on_click: T,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            class="min-h-12 text-neutral-200 hover:bg-background p-2 border-b last:border-none border-zinc-50/10"
            on:click=on_click
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SearchEntry(#[prop(default = "Search".to_string())] placeholder: String) -> impl IntoView {
    let (value, set_value) = create_signal("".to_string());
    let (length, set_length) = create_signal(0);

    create_effect(move |_| {
        set_length(value().len());
    });

    let id = format!("default-search-{}", ObjectId::new());

    view! {
        <div class="max-w-md mx-auto">
            <label for=id.to_owned() class="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white">Search</label>
            <div class="relative">
                <button class="absolute inset-y-0 start-0 flex items-center ps-3 text-gray-400 hover:text-gray-200">
                    <Icon class="w-4 h-4 text-neutral-400 hover:text-neutral-200".to_string() name="search-loop".to_string() />
                </button>
                <input
                    type="search"
                    id=id
                    class="block w-full p-2 ps-10 text-sm bg-foreground text-neutral-200 placeholder-neutral-500 \
                        rounded-lg hover:bg-hover active:bg-foreground focus:outline-none focus:ring-2 \
                        focus:ring-blue-300/50 w-full"
                    placeholder=placeholder
                    on:input=move |ev| set_value(event_target_value(&ev))
                    prop:value=value
                />
                <button
                    type="button"
                    class="absolute end-2 bottom-2.5 bg-transparent focus:outline-none p-0"
                    class:hidden=move || length() == 0
                    on:click=move |_| set_value("".to_string())
                >
                    <Icon name="backspace".to_string() />
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn Icon(name: String, #[prop(optional)] class: String) -> impl IntoView {
    let xmlns = "http://www.w3.org/2000/svg";
    let default_class = format!("w-4 h-4 text-neutral-200 {}", class);

    {
        match name.as_str() {
            "backspace" => {
                view! {
                    <svg
                        class=default_class
                        xmlns=xmlns
                        fill="currentColor"
                        stroke="none"
                        viewBox="0 0 24 24"
                    >
                        <path d="M22 3H7c-.69 0-1.23.35-1.59.88L0 12l5.41 8.11c.36.53.9.89 1.59.89h15c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-3 12.59L17.59 17 14 13.41 10.41 17 9 15.59 12.59 12 9 8.41 10.41 7 14 10.59 17.59 7 19 8.41 15.41 12 19 15.59z"/>
                    </svg>
                }
            }
            "search-loop" => {
                view! {
                    <svg
                        class=default_class
                        xmlns=xmlns
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 20 20"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"
                        />
                    </svg>
                }
            }
            "edit" => {
                view! {
                    <svg
                        class=default_class
                        xmlns=xmlns
                        fill="currentColor"
                        stroke="none"
                        viewBox="0 0 24 24"
                    >
                        <path
                            d="M3 17.25V21h3.75L17.81 9.94l-3.75-3.75L3 17.25zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"
                        />
                    </svg>

                }
            }
            "angle-right" => {
                view! {
                    <svg
                        class=default_class
                        xmlns=xmlns
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 20 20"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="3"
                            d="m9 5 7 7-7 7" />

                    </svg>

                }
            }
            "angle-left" => {
                view! {
                    <svg
                        class=default_class
                        xmlns=xmlns
                        fill="none"
                        stroke="currentColor"
                        viewBox="0 0 20 20"
                    >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="3"
                            d="m15 19-7-7 7-7" />
                    </svg>

                }
            }
            _ => {
                view! {
                    <svg
                        class=default_class
                        xmlns=xmlns
                        fill="none"
                        stroke="none"
                        viewBox="0 0 20 20"
                    >
                        <path></path>
                    </svg>
                }
            }
        }
    }
}

#[component]
pub fn StoreActionRow<T>(value: ReadSignal<Store>, on_click: T) -> impl IntoView
where
    T: FnMut(leptos::ev::MouseEvent) + 'static,
{
    let (city, set_city) = create_signal("".to_string());
    let (country, set_country) = create_signal("".to_string());
    let (name, set_name) = create_signal("".to_string());
    let (address, set_address) = create_signal("".to_string());

    create_effect(move |_| {
        let city = value().city.unwrap_or("".to_string()).to_string();
        let country = value().country.unwrap_or("".to_string()).to_string();
        let name = value().name.unwrap_or("".to_string()).to_string();
        let address = value().address.unwrap_or("".to_string()).to_string();

        set_city(city);
        set_country(country);
        set_name(name);
        set_address(address);
    });

    view! {
        <div
            class="flex items-center relative bg-foreground text-neutral-200 placeholder-neutral-500 rounded-lg \
            hover:bg-hover active:bg-foreground focus:outline-none focus:ring-2 focus:ring-blue-300/50 px-3 py-0.5 w-full \
            appearance-none"
            on:click=on_click
        >
            <div class="mr-3">
                <label class="text-md text-neutral-200 block">
                    {name}
                </label>
                <span class="text-xs text-neutral-400">
                    <span>{city}</span>, <strong>{country}</strong>
                </span>
            </div>
            <div class="grow text-right pr-4">
                {address}
            </div>
            <button
                type="button"
                class="absolute inset-y-0 end-2 bg-transparent focus:outline-none p-0"
            >
                <Icon name="angle-right".to_string() />
            </button>
        </div>
    }
}

#[component]
pub fn Product<T>(value: ReadSignal<Product>, on_change: T) -> impl IntoView
where
    T: FnMut(leptos::ev::Event) + 'static,
{
    let (r#ref, set_ref) = create_signal("".to_string());
    let (name, set_name) = create_signal("".to_string());
    let (price, set_price) = create_signal(Money::default());

    create_effect(move |_| {
        let r#ref = value().r#ref.unwrap_or("".to_string()).to_string();
        let name = value().name.unwrap_or("".to_string()).to_string();
        let price = value().price.unwrap_or(Money {
            currency: Some("".to_string()),
            amount: Some(0),
        });

        set_ref(r#ref);
        set_name(name);
        set_price(price);
    });

    view! {
        <>
            <EntryRow
                label="Ref".to_string()
                value=r#ref
                on_input=move |ev| set_ref(event_target_value(&ev))
            />

            <EntryRow
                label="Name".to_string()
                value=name
                on_input=move |ev| set_name(event_target_value(&ev))
            />

            <MoneyActionRow
                label="Price".to_string()
                value=price
                on_change=set_price
            />
        </>
    }
}

#[component]
pub fn MoneyActionRow<T>(label: String, value: ReadSignal<Money>, on_change: T) -> impl IntoView
where
    T: FnMut(Money) + 'static,
{
    let (amount, set_amount) = create_signal(0.0);
    let (currency, set_currency) = create_signal("USD".to_string());

    let options = UseIntlNumberFormatOptions::default()
        .locale("co-CO")
        .style(NumberStyle::Currency)
        .currency("USD");

    let number_format = use_intl_number_format(options);

    let (formated_amount, set_formated_amount) = create_signal(number_format.format::<f64>(amount));

    create_effect(move |_| {
        let amount = value().amount.unwrap_or(0) as f64;
        let currency = value().currency.unwrap_or("".to_string());

        set_amount(amount);
        set_currency(currency.clone());
    });

    create_effect(move |_| {
        let options = UseIntlNumberFormatOptions::default()
            .locale("co-CO")
            .style(NumberStyle::Currency)
            .maximum_significant_digits(3)
            .currency(currency());

        let number_format = use_intl_number_format(options);

        set_formated_amount(number_format.format::<f64>(amount));
    });

    view! {
        <div
            class="flex items-center relative bg-foreground text-neutral-200 placeholder-neutral-500 rounded-lg \
            hover:bg-hover active:bg-foreground focus:outline-none focus:ring-2 focus:ring-blue-300/50 px-3 py-0.5 w-full \
            appearance-none"
        >
            <div class="mr-3 py-3">
                <label class="text-md text-neutral-200 block">
                    {label}
                </label>
            </div>
            <div class="grow text-right pr-4">
                {formated_amount}
            </div>
            <button
                type="button"
                class="absolute inset-y-0 end-2 bg-transparent focus:outline-none p-0"
            >
                <Icon name="angle-right".to_string() />
            </button>
        </div>
    }
}

#[component]
pub fn WindowAction<T, U>(
    #[prop(default = "Cancel".to_string())] cancel_label: String,
    #[prop(default = "Accept".to_string())] accept_label: String,
    on_cancel: T,
    on_accept: U,
) -> impl IntoView
where
    T: FnMut(leptos::ev::MouseEvent) + 'static,
    U: FnMut(leptos::ev::MouseEvent) + 'static,
{
    view! {
        <div class="flex justify-between">
            <Button
                label=cancel_label
                on_click=on_cancel
            />

            <Button
                label=accept_label
                primary=true
                on_click=on_accept
            />
        </div>
    }
}
