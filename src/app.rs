use crate::error_template::{AppError, ErrorTemplate};
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


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rabbitnook.css"/>

        // sets the document title
        <Title text="RabbitNook"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
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

    view! {
            <img src="/rabbitnook.jpg" alt="Rabbitnook"/>
            <p>You are visitor number: { visitor_number }</p>
    }
}
