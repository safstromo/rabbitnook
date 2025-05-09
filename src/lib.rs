pub mod app;
mod components;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    use leptos::mount;
    console_error_panic_hook::set_once();
    mount::hydrate_body(App);
}
