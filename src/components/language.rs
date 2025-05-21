use clsx::clsx;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{HiChevronDown, HiGlobeAlt};
use dioxus_free_icons::Icon;
use dioxus_i18n::unic_langid::langid;

use crate::store::{use_language, LanguageItem};

#[component]
pub fn Language() -> Element {
    let (currentLanguage, set_language) = use_language();

    let select_language = use_callback(move |lang: LanguageItem| {
        tracing::debug!("Clicked! Event: {lang:?}\n",);
        set_language(lang.value.to_string());
    });

    let languages = vec![
        LanguageItem {
            value: langid!("en"),
            label: "English".to_string(),
        },
        LanguageItem {
            value: langid!("cn"),
            label: "中 文".to_string(),
        },
    ];

    rsx!(
        div { class: clsx!("ui-dropdown", "ui-dropdown-center"),
            div {
                tabindex: "0",
                role: "button",
                class: "ui-btn ui-btn-ghost px-1.5  menu-dropdown-toggle",
                Icon { icon: HiGlobeAlt }
                Icon { icon: HiChevronDown }
            }
            ul {
                tabindex: "0",
                class: "ui-dropdown-content ui-menu  bg-base-100 rounded-box z-1 shadow-sm",
                ul { class: "ui-menu p-0 w-full",

                    {
                        languages
                            .iter()
                            .cloned()
                            .map(|lang| {
                                let active = lang.value.to_string() == currentLanguage.read().as_ref();
                                rsx! {
                                    li {
                                        onclick: move |_| {
                                            if !active {
                                                select_language(lang.clone())
                                            }
                                        },
                                        a { class: clsx!((active, "ui-menu-active")), "{lang.label}" }
                                    }
                                }
                            })
                    }
                }
            }
        }
    )
}
