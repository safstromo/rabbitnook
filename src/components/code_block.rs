use leptos::prelude::*;

#[component]
pub fn CodeBlock(
    class: &'static str,
    code: &'static str,
    #[prop(default = false)] inline: bool,
) -> impl IntoView {
    view! {
        <div class="code-block">
            {if inline {
                view! { <code class=class>{code}</code> }.into_any()
            } else {
                view! {
                    <pre>
                        <code class=class>{code}</code>
                    </pre>
                }
                    .into_any()
            }}
        </div>
    }
}
