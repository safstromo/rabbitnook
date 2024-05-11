use crate::{
    components::terminal::{TerminalHistory, TerminalInput},
    error_template::{AppError, ErrorTemplate},
};
use lazy_static::lazy_static;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::sync::atomic::{AtomicU32, Ordering};

// Define a struct to hold the visitor counter
struct VisitorCounter {
    counter: AtomicU32,
}

impl VisitorCounter {
    fn new() -> Self {
        Self {
            counter: AtomicU32::new(0),
        }
    }

    fn increment(&self) -> u32 {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }
}

// Lazily initialize the visitor counter using lazy_static
lazy_static! {
    static ref VISITOR_COUNTER: VisitorCounter = VisitorCounter::new();
}

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
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Increment the counter using the AtomicU32
    let visitor_number = VISITOR_COUNTER.increment();
    let (command_history, set_command_history) = create_signal(vec![]);

    view! {
        <div class="flex h-screen w-screen bg-base">
            <div class="w-1/2 flex flex-col justify-center items-center">
                <NameHeader/>
            </div>
            <section class="w-1/2 flex flex-col justify-center items-center">
                <div class="flex flex-col border shadow-md shadow-black border-peach bg-base rounded-md w-5/6 h-5/6">
                    <p class="text-white m-2">"Type 'help' for available commands"</p>
                    <TerminalHistory command_history=command_history/>
                    <TerminalInput set_command_history=set_command_history command_history=command_history/>
                </div>
                <p class="text-sky mt-4">You are visitor number: {visitor_number}</p>
            </section>
        </div>
    }
}

#[component]
fn NameHeader() -> impl IntoView {
    view! {
        <section class="ml-4 items-start">
            <h1 class="text-8xl text-blue">hey there, Im</h1>
            <h2 class="text-7xl mt-2 font-semibold text-maroon">Oliver Säfström</h2>
            <div class="h-1 m-1 w-52 bg-sky"></div>
            <h3 class="text-xl ml-4 text-green">fullstack developer</h3>
        </section>
        <img class="rounded-full w-2/3 m-7" src="/portrait.png" alt="Portrait"/>
    }
}
