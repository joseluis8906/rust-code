use leptos::*;

#[component]
pub fn Desktop(children: Children) -> impl IntoView {
    view! {
        <div class="conainer mx-auto p-1 sm:p-2 md:p-4 w-screen h-screen flex items-center justify-center desktop">
            {children()}
        </div>
    }
}

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div
            class="w-11/12 sm:w-10/12 md:w-3/5 bg-background shadow-md shadow-gray-900 mx-auto rounded-lg grid gap-5 \
                    grid-cols-1 py-4 sm:py-8 lg:py-16 px-4 sm:px-16 lg:px-32 xl:px-48 2xl:px-64"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn Text(
    #[prop(default = "Input Text")] label: &'static str,
    on_input: WriteSignal<String>,
    value: ReadSignal<String>,
) -> impl IntoView {
    view! {
        <div class="relative">
            <input
                id="floating_filled"
                class="block bg-foreground text-neutral-200 placeholder-neutral-500 rounded-lg hover:bg-hover \
                        active:bg-foreground focus:outline-none focus:ring-2 focus:ring-blue-300/50 pt-5 pb-2.5 px-3 \
                        mt-1 w-full appearance-none peer"
                type="text"
                placeholder=" "
                on:input=move |ev| {
                    let val = event_target_value(&ev);
                    on_input(val);
                }
                prop:value=value
            />

            <label for="floating_filled" class="absolute text-md text-neutral-400 duration-300 transform -translate-y-4 \
                    scale-75 top-4 z-10 origin-[0] start-2.5 peer-focus:text-neutral-200 peer-placeholder-shown:scale-100 \
                    peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-4 \
                    rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto"
            >{label}</label>
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
pub fn Button<T>(label: &'static str, on_click: T) -> impl IntoView
where
    T: FnMut(leptos::ev::MouseEvent) + 'static,
{
    view! {
        <button
            class="bg-cyan-600/50 text-neutral-200 rounded-lg shadow-md shadow-neutral-900 hover:bg-cyan-600/75 active:bg-cyan-600/50 p-2"
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
                    // <img src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80" alt="" class="h-5 w-5 flex-shrink-0 rounded-full" />
                    // <span class="ml-3 block truncate">Tom Cook</span>
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
                        // <img src="https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80" alt="" class="h-5 w-5 flex-shrink-0 rounded-full" />
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
pub fn Dialog() -> impl IntoView {
    view! {}
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
pub fn ActionRow() -> impl IntoView {
    view! {}
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
pub fn EntryRow() -> impl IntoView {
    view! {}
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
pub fn NavigationView() -> impl IntoView {
    view! {}
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
pub fn WindowTitle() -> impl IntoView {
    view! {}
}

#[component]
pub fn HeaderBar() -> impl IntoView {
    view! {}
}

#[component]
pub fn Window() -> impl IntoView {
    view! {}
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
        <div>
            {children()}
        </div>
    }
}

#[component]
pub fn ListBoxRow() -> impl IntoView {
    view! {}
}

#[component]
pub fn SearchEntry(#[prop(default = "Search")] placeholder: &'static str) -> impl IntoView {
    let (value, set_value) = create_signal("".to_string());
    let (length, set_length) = create_signal(0);

    create_effect(move |_| {
        set_length(value().len());
    });

    view! {
        <form class="max-w-md mx-auto">
            <label for="default-search" class="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white">Search</label>
            <div class="relative">
                <button class="absolute inset-y-0 start-0 flex items-center ps-3 text-gray-400 hover:text-gray-200">
                    <Icon class="w-4 h-4 text-neutral-400 hover:text-neutral-200" name="search-loop" />
                </button>
                <input
                    type="search"
                    id="default-search"
                    class="block w-full p-2 ps-10 text-sm bg-foreground text-neutral-200 placeholder-neutral-500 \
                        rounded-lg hover:bg-hover active:bg-foreground focus:outline-none focus:ring-2 \
                        focus:ring-blue-300/50 w-full"
                    placeholder=placeholder
                    on:input=move |ev| set_value(event_target_value(&ev))
                    prop:value=value
                />
                <button
                    type="button"
                    class="absolute end-2 bottom-2.5 bg-transparent focus:outline-none font-medium rounded-lg text-sm p-0"
                    class:hidden=move || length() == 0
                    on:click=move |_| set_value("".to_string())
                >
                    <Icon name="backspace" />
                </button>
            </div>
        </form>
    }
}

#[component]
pub fn Icon(
    name: &'static str,
    #[prop(default = "w-4 h-4 text-neutral-200")] class: &'static str,
) -> impl IntoView {
    let xmlns = "http://www.w3.org/2000/svg";

    view! {
            {
                match name {
                    "backspace" => {
                        view! {
                            <svg
                                class=class
                                xmlns=xmlns
                                fill="currentColor"
                                stroke="none"
                                viewBox="0 0 24 24"
                            >
                                <path d="M22 3H7c-.69 0-1.23.35-1.59.88L0 12l5.41 8.11c.36.53.9.89 1.59.89h15c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-3 12.59L17.59 17 14 13.41 10.41 17 9 15.59 12.59 12 9 8.41 10.41 7 14 10.59 17.59 7 19 8.41 15.41 12 19 15.59z"/>
                            </svg>
                        }
                    },
                    "search-loop" => {
                        view! {
                            <svg
                                class=class
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
                    },
                    _ => {
                        view!{
                            <svg
                                class=class
                                xmlns=xmlns
                                fill="none"
                                stroke="none"
                                viewBox="0 0 20 20"
                            >
                                <path></path>
                            </svg>
                        }
                    },
                }
            }
    }
}
