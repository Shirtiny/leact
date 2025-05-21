use dioxus::prelude::*;
use dioxus_i18n::t;

#[component]
pub fn FileDrop() -> Element {
    rsx! {

        fieldset { class: "ui-fieldset ui-file-input-md",
            legend { class: "ui-fieldset-legend", {t!("select_file_legend")} }
            input { r#type: "file", class: "ui-file-input" }
            label { class: "ui-label", {t!("select_file_desc", size : "1 GB")} }
        }
    }
}
