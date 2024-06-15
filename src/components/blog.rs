use leptos::{component, view, IntoView};

use crate::components::{links::Links, name_header::NameHeader};

#[component]
pub fn Blog() -> impl IntoView {
    view! {
        <div class="flex md:flex-row flex-col min-h-screen w-full bg-base items-center justify-center">
            <div class="md:w-1/2 w-5/6 flex flex-col justify-center items-center">
                <NameHeader/>
                <Links/>
            </div>
        </div>
    }
}
