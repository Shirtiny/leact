use dioxus::{
    hooks::{use_callback, use_context, use_context_provider},
    logger::tracing,
    prelude::Callback,
    signals::{Signal, Writable},
};
use dioxus_sdk::storage::{use_storage, LocalStorage};

use super::global::{DEFAULT_THEME, STORAGE_THEME};

#[derive(Clone, Copy)]
pub struct ThemeState {
    pub current_theme: Signal<String>,
}

pub fn use_theme_provider() -> ThemeState {
    let current_theme =
        use_storage::<LocalStorage, _>(STORAGE_THEME.into(), || DEFAULT_THEME.to_string());

    let state = use_context_provider(|| ThemeState { current_theme });

    return state;
}

pub fn use_theme() -> (Signal<std::string::String>, Callback) {
    let ThemeState { mut current_theme } = use_context::<ThemeState>();

    let toggle_theme = use_callback(move |_| {
        tracing::debug!("toggle_theme {}", &current_theme);
        let light: String = "cupcake".into();
        let dark: String = "business".into();

        current_theme.set(if current_theme.to_string() == dark {
            light
        } else {
            dark
        });
    });

    return (current_theme, toggle_theme);
}
