use crate::components::{
    blog::Blog,
    links::Links,
    name_header::NameHeader,
    terminal::{TerminalHistory, TerminalInput},
};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/rabbitnook.css" />
        <Link rel="preconnect" href="https://fonts.googleapis.com" />
        <Link rel="preconnect" href="https://fonts.gstatic.com" />
        <Link
            href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&display=swap"
            rel="stylesheet"
        />
        <Link
            rel="stylesheet"
            href="//unpkg.com/@catppuccin/highlightjs@0.2.2/css/catppuccin-mocha.css"
        />
        <Script
            src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.11.1/highlight.min.js"
            integrity="sha512-EBLzUL8XLl+va/zAsmXwS7Z2B1F9HUHkZwyS/VKwh3S7T/U0nF4BaU29EP/ZSf6zgiIxYAnKLu6bJ8dqpmX5uw=="
            crossorigin="anonymous"
            referrerpolicy="no-referrer"
        />

        // sets the document title
        <Title text="RabbitNook" />

        <Router>
            <main>
                <Routes fallback=|| "Not Found">
                    <Route path=path!("") view=HomePage />
                    <Route path=path!("/blog") view=Blog />
                // <Route path=path!("/blog/esp32-relay") view=Blog/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (command_history, set_command_history) = signal(vec![]);

    let input_element = NodeRef::new();

    view! {
        <div class="flex md:flex-row flex-col min-h-screen w-full bg-base items-center justify-center">
            <div class="md:w-1/2 w-5/6 flex flex-col justify-center items-center">
                <NameHeader />
                <Links />
                <div class="w-44 mb-4">
                    <a href="https://ko-fi.com/safstromo" target="_blank">
                        <img src="kofi_button_blue.png" />
                    </a>
                </div>

                <nav class="my-4 text-white text-2xl font-semibold hover:border-peach border-base border-2 rounded-lg">
                    <a class="mx-1 text-white hover:text-maroon" href="/blog">
                        Blog
                    </a>
                </nav>
            </div>
            <section class="md:w-1/2 w-5/6 md:h-screen flex flex-col justify-center items-center">
                <div
                    class="flex flex-col border shadow-md shadow-black border-peach rounded-md bg-base w-full md:w-5/6 min-h-96 h-5/6"
                    on:click=move |_| {
                        let _ = input_element.get().expect("Input shoud be there to focus").focus();
                    }
                >

                    <p class="text-white m-2">"Type 'help' for available commands."</p>
                    <TerminalHistory command_history=command_history />
                    <TerminalInput
                        input_element=input_element
                        set_command_history=set_command_history
                        command_history=command_history
                    />

                </div>
            </section>
        </div>
    }
}
