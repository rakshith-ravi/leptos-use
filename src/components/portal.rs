use crate::core::ElementMaybeSignal;
use leptos::*;

///
///
/// ## Demo
///
/// [Link to Demo](https://github.com/Synphonyte/leptos-use/tree/main/examples/portal)
///
/// ## Usage
///
/// ```
/// # use leptos::*;
/// # use leptos_use::components::Portal;
/// #
/// # #[component]
/// # fn Demo() -> impl IntoView {
/// #
/// # view! { }
/// # }
/// ```
#[component]
pub fn Portal<El, T>(
    #[prop(optional)] mount: El,
    #[prop(default = false)] use_shadow: bool,
    #[prop(default = false)] is_svg: bool,
    children: Children,
) -> impl IntoView
where
    El: Into<ElementMaybeSignal<T, web_sys::Element>>,
    T: Into<web_sys::Element> + Clone + 'static,
{
    create_effect(move |_| {})
}
