use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos_router::components::A;

use crate::components::links::Links;

#[component]
pub fn Blog() -> impl IntoView {
    view! {
        <div class="flex md:flex-row flex-col min-h-screen w-full bg-base items-center justify-center">
            <div class="md:w-2/3 w-5/6 flex flex-col justify-center items-center">

                <h1 class="my-10 underline">Posts</h1>
                <A href="/blog/nix-frame-hack">
                    <div class="flex flex-col mt-10 border rounded-md w-full">
                        <h1 class="mx-4 mt-2 text-xl hover:text-maroon">NixPlay frame hacking</h1>
                        <p class="mx-4 my-2">April 26, 2025</p>

                    </div>
                </A>
                <A href="/blog/esp32-relay">
                    <div class="flex flex-col my-10 border rounded-md w-full">
                        <h1 class="mx-4 mt-2 text-xl hover:text-maroon">
                            "Rust ""❤️"" Esp32 remote relay using AWS IoT"
                        </h1>
                        <p class="mx-4 my-2">June 20, 2024</p>
                    </div>
                </A>
                <Links />
                <div class="w-44 mb-6">
                    <a href="https://ko-fi.com/safstromo" target="_blank">
                        <img src="../kofi_button_blue.png" />
                    </a>
                </div>
                <A href="/">
                    <p class="underline">Home</p>
                </A>
            </div>
        </div>
    }
}
