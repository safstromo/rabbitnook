use leptos::prelude::*;
use leptos_meta::Script;

use crate::components::links::Links;

#[component]
pub fn NixFrameHack() -> impl IntoView {
    let policy_json_all = r#"
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": "iot:*",
      "Resource": "*"
    }
  ]
}
"#;

    let render_call = r#"
if (window.hljs) {
    hljs.highlightAll();
} else {
    document.querySelector('#hljs-src')
        .addEventListener('load', (e) => { hljs.highlightAll() }, false);
};"#;

    let (script, set_script) = signal(None::<String>);
    let blog_view = move || {
        Suspend::new(async move {
            Effect::new(move |_| {
                set_script.set(Some(render_call.to_string()));
            });
            view! {
                <pre>
                    <code class="json">{policy_json_all}</code>
                </pre>
                {move || {
                    script
                        .get()
                        .map(|script| {
                            view! { <Script>{script}</Script> }
                        })
                }}
            }
        })
    };
    view! {
        <Script id="hljs-src" async_="true" src="/highlight.min.js" />

        <div class="flex md:flex-row flex-col min-h-screen w-full bg-base items-center justify-center">
            <div class="md:w-2/3 w-5/6 flex flex-col justify-center items-center">
                <Suspense fallback=move || view! { <p>"Loading blog..."</p> }>{blog_view}</Suspense>
                <Links />
                <div class="w-44 mb-6">
                    <a href="https://ko-fi.com/safstromo" target="_blank">
                        <img src="../kofi_button_blue.png" />
                    </a>
                </div>
            </div>
        </div>
    }
}
