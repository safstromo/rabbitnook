use leptos::{component, view, IntoView};

#[component]
pub fn Links() -> impl IntoView {
    view! {
        <section class="flex justify-between items-center w-40 m-4">
            <a href="https://github.com/safstromo" target="_blank">
                <img class="w-10" src="/github-mark-white.svg" alt="Github Link"/>
            </a>
            <a href="https://www.linkedin.com/in/safstromo" target="_blank">
                <img class="w-10" src="/linkedin-white.svg" alt="Linkedin Link"/>
            </a>
            <a href="mailto: safstrom.oliver@gmail.com" target="_blank">
                <img class="w-10" src="/gmail.svg" alt="Gmail Link"/>
            </a>

        </section>
    }
}
