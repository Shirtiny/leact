use dioxus::{
    hooks::{use_callback, use_context, use_context_provider, use_effect},
    logger::tracing,
    prelude::Callback,
    signals::{Readable, Signal, Writable},
};
use dioxus_i18n::{
    prelude::*,
    unic_langid::{langid, LanguageIdentifier},
};
use dioxus_sdk::storage::*;

use super::global::STORAGE_LANGUAGE;

#[derive(Debug, PartialEq, Clone)] // 使结构体可以打印并且支持比较
pub struct LanguageItem {
    pub value: LanguageIdentifier,
    pub label: String,
}

#[derive(Clone, Copy)]
pub struct LanguageState {
    pub current_language: Signal<String>,
}

pub fn use_init_language() {
    use_init_i18n(|| {
        I18nConfig::new(langid!("en"))
            .with_locale((langid!("cn"), include_str!("../../i18n/cn.ftl")))
            .with_locale((langid!("en"), include_str!("../../i18n/en.ftl")))
            .with_fallback(langid!("en"))
    });
}

pub fn use_language_provider() -> LanguageState {
    let current_language =
        use_storage::<LocalStorage, _>(STORAGE_LANGUAGE.into(), || "en".to_string());

    let mut i18n = i18n();

    use_effect(move || {
        let lang = LanguageIdentifier::from_bytes(current_language.read().as_bytes())
            .expect("validated language identifier string");
        i18n.set_language(lang);
    });

    let state = use_context_provider(|| LanguageState { current_language });

    return state;
}

pub fn use_language() -> (Signal<std::string::String>, Callback<std::string::String>) {
    let LanguageState {
        mut current_language,
    } = use_context::<LanguageState>();

    let set_language = use_callback(move |v: String| {
        current_language.set(v.clone());
    });

    // 打印currentLanguage
    tracing::debug!("currentLanguage: {}", current_language);

    return (current_language, set_language);
}
