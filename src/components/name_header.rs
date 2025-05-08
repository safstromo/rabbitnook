use leptos::{component, view, IntoView};

use leptos::prelude::*;

#[component]
pub fn NameHeader() -> impl IntoView {
    view! {
        <section class="flex flex-col m-4 md:p-4  items-start">
            <h1 class="text-5xl md:text-8xl text-blue">hey there, Im</h1>
            <h2 class="text-4xl md:text-7xl mt-2 font-semibold text-maroon">Oliver Säfström</h2>
            <div class="h-1 m-1 w-40 md:w-60 bg-sky"></div>
            <h3 class="md:text-xl ml-2 md:ml-4 text-green">fullstack developer</h3>
        </section>
        <img class="rounded-full w-2/3 max-w-[360px] m-7" src="/portrait.png" alt="Portrait" />
    }
}
