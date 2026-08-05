use leptos::prelude::*;
use leptos_meta::Script;
use leptos_router::components::A;

use crate::components::{kofi::KofiButton, links::Links};

const RENDER_CALL: &str = r#"
if (window.hljs) {
    hljs.highlightAll();
} else {
    document.querySelector('#hljs-src')
        .addEventListener('load', (e) => { hljs.highlightAll() }, false);
};"#;

#[component]
pub fn BlogLayout(
    children: ChildrenFn,
    #[prop(default = false)] overflow_x: bool,
) -> impl IntoView {
    let (script, set_script) = signal(None::<String>);

    let blog_view = move || {
        let children = children.clone();
        Suspend::new(async move {
            Effect::new(move |_| {
                set_script.set(Some(RENDER_CALL.to_string()));
            });
            view! {
                <div class="flex flex-col min-h-screen w-full max-w-5xl bg-base items-start justify-center">
                    {children()}
                    {move || { script.get().map(|script| view! { <Script>{script}</Script> }) }}
                </div>
            }
        })
    };

    let outer_class = if overflow_x {
        "flex md:flex-row flex-col min-h-screen w-full bg-base items-center justify-center overflow-x-auto"
    } else {
        "flex md:flex-row flex-col min-h-screen w-full bg-base items-center justify-center"
    };

    view! {
        <Script id="hljs-src" async_="true" src="/highlight.min.js" />
        <div class=outer_class>
            <div class="md:w-2/3 w-5/6 flex flex-col justify-center items-center mb-4">
                <Suspense fallback=move || view! { <p>"Loading blog..."</p> }>{blog_view}</Suspense>
                <Links />
                <KofiButton margin="mb-6" />
                <A href="/">
                    <p class="underline">Home</p>
                </A>
            </div>
        </div>
    }
}
