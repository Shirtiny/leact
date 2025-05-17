use dioxus::prelude::*;

#[component]
pub fn Canvas() -> Element {
    let mut canvas_element = use_signal(|| None);

    rsx! {
        canvas { onmounted: move |element| canvas_element.set(Some(element.data())) }
    }
}
