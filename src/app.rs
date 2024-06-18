use crate::{
    components::{
        blog::Blog,
        links::Links,
        name_header::NameHeader,
        terminal::{TerminalHistory, TerminalInput},
    },
    error_template::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/rabbitnook.css"/>
        <Link rel="preconnect" href="https://fonts.googleapis.com"/>
        <Link rel="preconnect" href="https://fonts.gstatic.com"/>
        <Link
            href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&display=swap"
            rel="stylesheet"
        />
        <Link
            rel="stylesheet"
            href="//unpkg.com/@catppuccin/highlightjs@0.2.2/css/catppuccin-mocha.css"
        />
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/rust.min.js"/>
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/json.min.js"/>
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/dockerfile.min.js"/>
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/es/languages/bash.min.js"/>
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"/>

        // sets the document title
        <Title text="RabbitNook"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/blog" view=Blog/>
                    <Route path="/blog/esp32-relay" view=Blog/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (command_history, set_command_history) = create_signal(vec![]);
    let input_element: NodeRef<html::Input> = create_node_ref();

    view! {
        <div class="flex md:flex-row flex-col min-h-screen w-full bg-base items-center justify-center">
            <div class="md:w-1/2 w-5/6 flex flex-col justify-center items-center">
                <NameHeader/>
                <Links/>
                <div class="w-44 mb-4">
                    <a href="https://ko-fi.com/safstromo" target="_blank">
                        <img src="kofi_button_blue.png"/>
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
                    <TerminalHistory command_history=command_history/>
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
