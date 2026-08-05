use leptos::prelude::*;

#[component]
pub fn KofiButton(#[prop(default = "mb-4")] margin: &'static str) -> impl IntoView {
    view! {
        <div class=format!("w-44 {margin}")>
            <a href="https://ko-fi.com/safstromo" target="_blank">
                <img src="/kofi_button_blue.png" alt="Support me on Ko-fi" />
            </a>
        </div>
    }
}
