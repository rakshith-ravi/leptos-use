# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 

### New Functions 🚀

- `signal_debounced`
- `signal_throttled`

### New Component 🚀
- `Portal`

### Braking Changes 🛠

- Leptos version is now 0.5
- No `cx: Scope` params are supported/needed anymore because of the changes in Leptos.
  Please check the release notes of Leptos 0.5 for how to upgrade.
- `watch` is now removed in favor of `leptos::watch` and will be removed in a future release.
`watch_with_options` will continue to exist.
- `use_websocket`:
  - takes  now a `&str` instead of a `String` as its `url` parameter.
  -  The `ready_state` return type is now renamed to `ConnectionReadyState` instead of `UseWebSocketReadyState`.
  - The returned signals  `ready_state`, `message`, `message_bytes` have now the type 
    `Signal<...>` instead of `ReadSignal<...>` to make them more consistent with other functions.
  - The options `reconnect_limit` and `reconnect_interval` now take a `u64` instead of `Option<u64>` to improve DX.
  - The option `manual` has been renamed to `immediate` to make it more consistent with other functions.
    To port please note that `immediate` is the inverse of `manual` (`immediate` = `!manual`).
- `use_color_mode`:
  - The optional `on_changed` handler parameters have changed slightly. Please refer to the docs for more details.
- Throttled or debounced functions cannot be `FnOnce` anymore.
- All traits `ClonableFn...` have been removed.

### Other Changes 🔥

- Callbacks in options don't require to be cloneable anymore
- Callback in `use_raf_fn` doesn't require to be cloneable anymore

## [0.6.2] - 2023-08-03

### Fixes 🍕

- `use_event_listener_with_options` removes the handlers now correctly.

## [0.6.1] - 2023-08-03

### Fixes 🍕

- `use_storage` now uses `.get_untracked()` to avoid warnings.

## [0.6.0] - 2023-07-17

### New Functions 🚀

- `use_draggable`
- `use_to_string`
- `is_err`
- `is_ok`
- `is_none`
- `is_some`
- `use_raf_fn`

### Braking Changes 🛠

- The following functions now accept a `MaybeRwSignal` as their initial/default value which means
  you can use a synchronized `RwSignal` in those places.
    - `use_color_mode`
    - `use_cycle_list`
    - `use_favicon`
    - `use_storage`
    - `use_local_storage`
    - `use_session_storage`
- Instead of returning `ReadSignal`, the following functions now return `Signal`.
  - `use_color_mode`
  - `use_favicon`
  - `use_storage`
  - `use_local_storage`
  - `use_session_storage`

### Fixes 🍕

- `use_drop_zone` now uses `.get_untracked()` in event handlers

## [0.5.0] - 2023-07-15

### New Functions 🚀

- `use_drop_zone`
- `use_websocket` (thanks @sectore)
- `use_intl_number_format`

### Changes 🔥

- Crate is ready for Server-Side Rendering. Enable feature `ssr` like you do for `leptos`.

## [0.4.1] - 2023-07-07

### New Functions 🚀

- `use_window_focus`
- `use_window_scroll`
- `use_document_visibility`

## [0.4.0] - 2023-07-03

### Braking Changes 🛠

- Required `leptos` version is now 0.4
- Following the changes in `leptos` there is no longer a `stable` crate feature required in order to use this library
  with a stable toolchain.
  If you want to use it with a nightly toolchain you have to enable the `nightly` feature only on `leptos` directly.
  No change is required for `leptos-use` itself.

## [0.3.3] - 2023-06-24

### New Functions 🚀

- `use_color_mode`
- `use_cycle_list`
- `use_active_element`

### Changes 🔥

- You can now use this crate with the `stable` toolchain (thanks @lpotthast)
- Set leptos dependency to `default-features = false` in order to enable SSR.

## [0.3.2] - 2023-06-17

### New Functions 🚀

- `use_css_var`
- `use_element_hover`

## [0.3.1] - 2023-06-15

### New Functions 🚀

- `use_interval_fn`
- `use_interval`

## [0.3.0] - 2023-06-13

### Braking Changes 🛠

- `use_event_listener` no longer returns a `Box<dyn Fn()>` but a `impl Fn() + Clone`

### Changes 🔥

- You can now specify a `&str` or `Signal<String>` with CSS selectors wherever a node ref is accepted
- Callbacks of the following functions no longer require `Clone`
    - `use_resize_observer`
    - `use_intersection_observer`
- These functions now also accept multiple target elements in addition to a single one:
    - `use_resize_observer`
    - `use_intersection_observer`

### New Functions 🚀

- `whenever`
- `use_mutation_observer`
- `use_abs`
- `on_click_outside`

## [0.2.1] - 2023-06-11

### New Functions

- `use_intersection_observer`
- `use_element_visibility`

## [0.2.0] - 2023-06-11

### Braking Changes

- `watch` doesn't accept `immediate` as a direct argument anymore. This is only provided by the option variant.
- `watch` has now variant `watch_with_options` which allows for debouncing and throttling.

### New Functions

- `use_storage`
- `use_local_storage`
- `use_session_storage`
- `watch_debounced`
- `watch_throttled`
- `watch_pausable`
- `use_ceil`
- `use_round`
- `use_media_query`
- `use_preferred_dark`
- `use_preferred_contrast`
- `use_favicon`
- `use_breakpoints`

### Other Changes

- Function count badge in readme

## [0.1.8/9] - 2023-06-05

- Fixed documentation and doc tests running for functions behind `#[cfg(web_sys_unstable_apis)]`

## [0.1.7] - 2023-06-05

### New Function

- `use_element_size`

## [0.1.6] - 2023-06-03

### Changes

- Fixed documentation so all feature are documented

## [0.1.5] - 2023-06-03

### New Functions

- `use_floor`
- `use_max`
- `use_min`

### Changes

- New feature: `math` that has to be activated in order to use the math functions.

## [0.1.4] - 2023-06-02

### New Functions

- `use_supported`
- `use_resize_observer`
- `watch`
- `use_mouse`

### Changes

- Use the crate `default-struct-builder` to provide ergonimic function options.

## [0.1.3] - 2023-05-28

### New Functions

- `use_scroll`
- `use_debounce_fn`

### Other Changes

- Better and more beautiful demo integration into the guide.