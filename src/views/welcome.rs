// use crate::components::Wel;
use dioxus::prelude::*;
use dioxus_i18n::t;

const WELCOME_CSS: Asset = asset!("assets/styling/views/welcome.scss");

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Welcome() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: WELCOME_CSS }
        main { class: "page-welcome @container",

            ul { class: "ui-steps ui-steps-vertical @sm:ui-steps-horizontal",
                li { class: "ui-step ui-step-primary", {t!("upload")} }
                li { class: "ui-step ui-step-primary", {t!("edit_subtitles")} }
                li { class: "ui-step", {t!("export")} }
            }
        
        }
    }
}
