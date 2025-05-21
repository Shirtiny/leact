// use crate::components::Wel;
use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::FileDrop;

const WELCOME_CSS: Asset = asset!("assets/styling/views/welcome.scss");

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Welcome() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: WELCOME_CSS }
        main { class: "page-welcome h-full p-8 flex flex-col items-center justify-center",
            // video { src: asset!("/assets/eva-redit.mp4"), controls: true }

            div { class: "w-full flex flex-col justify-between max-w-3xl min-h-3/5 @container",
                div { class: "flex flex-col items-center",
                    h1 { class: "text-3xl",
                        {t!("welcome_to")}
                        span { class: "font-light p-2", "Leact" }
                    }
                    p { class: "mt-6 max-w-xl text-sm/6", {t!("welcome_description")} }
                }


                div { class: "flex justify-center my-16", FileDrop {} }
                div { class: "flex justify-center",
                    ul { class: "ui-steps ui-steps-vertical w-auto @sm:ui-steps-horizontal @sm:w-full",
                        li { class: "ui-step ui-step-primary", {t!("upload")} }
                        li { class: "ui-step", {t!("edit_subtitles")} }
                        li { class: "ui-step", {t!("export")} }
                    }
                }
            }
        }
    }
}
