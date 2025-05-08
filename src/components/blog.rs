use leptos::prelude::*;
use leptos::{component, view, IntoView};

use crate::components::{blogpages::esp32_relay::Esp32Relay, links::Links};

#[component]
pub fn Blog() -> impl IntoView {
    let js_func = r#"
                    document.addEventListener('DOMContentLoaded', (event) => {
                        hljs.highlightAll();
                    });
                    "#;

    view! {
        <script src="//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
        <script>{js_func}</script>
        <div class="flex md:flex-row flex-col min-h-screen w-full bg-base items-center justify-center">
            <div class="md:w-2/3 w-5/6 flex flex-col justify-center items-center">
                <Esp32Relay />
                <Links />
                <div class="w-44 mb-6">
                    <a href="https://ko-fi.com/safstromo" target="_blank">
                        <img src="kofi_button_blue.png" />
                    </a>
                </div>
            </div>
        </div>
    }
}
