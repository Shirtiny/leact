use dioxus::prelude::*;

#[component]
pub fn Wel() -> Element {
    rsx! {
        div { id: "welcome", class: "welcome",
            div { "Welcome to Leact!" }
            button { class: "btn", "Click Me" }
        }
    }
}
