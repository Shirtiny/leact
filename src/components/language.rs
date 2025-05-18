use clsx::clsx;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use dioxus_free_icons::icons::hi_solid_icons::{HiChevronDown, HiGlobeAlt};
use dioxus_free_icons::Icon;
use dioxus_i18n::prelude::*;

#[derive(Debug, PartialEq, Clone)] // 使结构体可以打印并且支持比较
struct LanguageItem {
    value: String,
    label: String,
}

#[component]
pub fn Language() -> Element {
    let mut i18n = i18n();
    let mut open = use_signal(|| false);

    let languages = vec![
        LanguageItem {
            value: "en".to_string(),
            label: "English".to_string(),
        },
        LanguageItem {
            value: "cn".to_string(),
            label: "中 文".to_string(),
        },
    ];

    let mut select_language = move |lang: &LanguageItem| {
        tracing::info!("Clicked! Event: {lang:?}\n",);
        let lang_id = lang.value.parse().unwrap(); // 使用 parse 将字符串转为 LanguageIdentifier
        i18n.set_language(lang_id);
        open.set(false);
    };

    let currentLanguage = i18n.language();

    // 打印currentLanguage
    tracing::debug!("currentLanguage: {}, {}", currentLanguage, open,);

    rsx!(
        div { class: clsx!("ui-dropdown", "ui-dropdown-end"),
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
                                let active = lang.value == currentLanguage.to_string();
                                rsx! {
                                    li { onclick: move |_| select_language(&lang),
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
