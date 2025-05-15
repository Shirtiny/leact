// use crate::components::Wel;
use dioxus::prelude::*;

const HOME_CSS : Asset = asset!("assets/styling/home.scss");

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }
        div { class: "page-home", "Welcome to leact" }
    }
}
