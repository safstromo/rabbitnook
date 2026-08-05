use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::{kofi::KofiButton, links::Links};

const POSTS: &[(&str, &str, &str)] = &[
    (
        "/blog/nix-frame-hack",
        "NixPlay frame hacking",
        "April 26, 2025",
    ),
    (
        "/blog/esp32-relay",
        "Rust ❤️ Esp32 remote relay using AWS IoT",
        "June 20, 2024",
    ),
];

#[component]
pub fn Blog() -> impl IntoView {
    view! {
        <div class="flex md:flex-row flex-col min-h-screen w-full bg-base items-center justify-center">
            <div class="md:w-2/3 w-5/6 flex flex-col justify-center items-center">

                <h1 class="my-10 underline">Posts</h1>
                {POSTS
                    .iter()
                    .enumerate()
                    .map(|(index, (href, title, date))| {
                        let spacing = if index == 0 { "mt-10" } else { "my-10" };
                        view! {
                            <A href=*href>
                                <div class=format!(
                                    "flex flex-col {spacing} border rounded-md w-full",
                                )>
                                    <h1 class="mx-4 mt-2 text-xl hover:text-maroon">{*title}</h1>
                                    <p class="mx-4 my-2">{*date}</p>
                                </div>
                            </A>
                        }
                    })
                    .collect_view()}
                <Links />
                <KofiButton margin="mb-6" />
                <A href="/">
                    <p class="underline">Home</p>
                </A>
            </div>
        </div>
    }
}
