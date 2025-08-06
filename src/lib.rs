pub mod app;
pub mod models;
pub mod components;

#[cfg(feature = "ssr")]
pub mod database;
pub mod server_functions;
pub mod auth;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
