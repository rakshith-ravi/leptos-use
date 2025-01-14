use crate::error_template::{AppError, ErrorTemplate};
use leptos::ev::{keypress, KeyboardEvent};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_use::storage::{use_local_storage, StringCodec};
use leptos_use::{
    use_color_mode, use_debounce_fn, use_event_listener, use_interval, use_intl_number_format,
    use_preferred_dark, use_timestamp, use_window, ColorMode, UseColorModeReturn,
    UseIntervalReturn, UseIntlNumberFormatOptions,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        <Title text="Leptos-Use SSR Example"/>

        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=|| view! { <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count, _) = use_local_storage::<i32, StringCodec>("count-state");
    let on_click = move |_| set_count.update(|count| *count += 1);

    let nf = use_intl_number_format(
        UseIntlNumberFormatOptions::default().locale("zh-Hans-CN-u-nu-hanidec"),
    );

    let zh_count = nf.format::<i32>(count);

    let (key, set_key) = create_signal("".to_string());

    // window() doesn't work on the server so we provide use_window()
    let _ = use_event_listener(use_window(), keypress, move |evt: KeyboardEvent| {
        set_key(evt.key())
    });

    let (debounce_value, set_debounce_value) = create_signal("not called");

    let debounced_fn = use_debounce_fn(
        move || {
            set_debounce_value("called");
        },
        2000.0,
    );
    debounced_fn();

    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();

    let timestamp = use_timestamp();

    let is_dark_preferred = use_preferred_dark();

    view! {
        <h1>Leptos-Use SSR Example</h1>
        <button on:click=on_click>Click Me: {count}</button>
        <p>Locale zh-Hans-CN-u-nu-hanidec: {zh_count}</p>
        <p>Press any key: {key}</p>
        <p>Debounced called: {debounce_value}</p>
        <p>Color mode: {move || format!("{:?}", mode.get())}</p>
        <button on:click=move |_| set_mode.set(ColorMode::Light)>Change to Light</button>
        <button on:click=move |_| set_mode.set(ColorMode::Dark)>Change to Dark</button>
        <button on:click=move |_| set_mode.set(ColorMode::Auto)>Change to Auto</button>
        <p>{timestamp}</p>
        <p>Dark preferred: {is_dark_preferred}</p>
        <LocalStorageTest/>
    }
}

#[component]
pub fn LocalStorageTest() -> impl IntoView {
    let UseIntervalReturn { counter, .. } = use_interval(1000);
    logging::log!("test log");
    let (state, set_state, ..) = use_local_storage::<String, StringCodec>("test-state");

    view! {
        <p>{counter}</p>
         <input
            class="block"
            prop:value=move || state.get()
            on:input=move |e| set_state.update(|s| *s = event_target_value(&e))
            type="text"
        />
    }
}
