use dioxus::{logger::tracing, prelude::*};
use dioxus_i18n::t;
use web_sys::wasm_bindgen::JsCast;

#[component]
pub fn FileUploader() -> Element {
    let handle_change = move |e: Event<FormData>| {
        #[cfg(feature = "web")]
        {
            use dioxus::web::WebEventExt;

            let web_event = e.as_web_event();
            let inputEl = web_event
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap();
            let file = inputEl.files().unwrap().get(0).unwrap();

            let object_url = web_sys::Url::create_object_url_with_blob(&file).unwrap();
            tracing::debug!("object_url {}", object_url);
        }
    };

    rsx! {
        fieldset { class: "ui-fieldset ui-file-input-md",
            legend { class: "ui-fieldset-legend", {t!("select_file_legend")} }
            input {
                r#type: "file",
                class: "ui-file-input",
                onchange: handle_change,
            }
            label { class: "ui-label", {t!("select_file_desc", size : "1 GB")} }
        }
    }
}
