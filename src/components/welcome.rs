use dioxus::prelude::*;

#[component]
pub fn Welcome() -> Element {
    rsx! {
        div { id: "welcome", class: "welcome",
            div { "Welcome to Leact!" }
        }
    }
}
