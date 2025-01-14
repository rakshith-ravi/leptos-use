use super::{use_storage_with_options, Codec, StorageType, UseStorageOptions};
use leptos::signal_prelude::*;

/// Reactive [SessionStorage](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage).
///
/// SessionStorages stores data in the browser that is deleted when the page session ends. A page session ends when the browser closes the tab. Data is not shared between pages. While data doesn't expire the user can view, modify and delete all data stored. Browsers allow 5MB of data to be stored.
///
/// Use [`use_local_storage`] to store data that is shared amongst all pages with the same origin and persists between page sessions.
///
/// ## Usage
/// See [`use_storage`] for more details on how to use.
pub fn use_session_storage<T, C>(
    key: impl AsRef<str>,
) -> (Signal<T>, WriteSignal<T>, impl Fn() + Clone)
where
    T: Clone + Default + PartialEq,
    C: Codec<T> + Default,
{
    use_storage_with_options(
        StorageType::Session,
        key,
        UseStorageOptions::<T, C>::default(),
    )
}

/// Accepts [`UseStorageOptions`]. See [`use_session_storage`] for details.
pub fn use_session_storage_with_options<T, C>(
    key: impl AsRef<str>,
    options: UseStorageOptions<T, C>,
) -> (Signal<T>, WriteSignal<T>, impl Fn() + Clone)
where
    T: Clone + PartialEq,
    C: Codec<T>,
{
    use_storage_with_options(StorageType::Session, key, options)
}
