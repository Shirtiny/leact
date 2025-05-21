use dioxus::signals::{Global, GlobalSignal};

pub const STORAGE_LANGUAGE: &str = "language";

// Globals are created the first time you access them with the closure you pass to Global::new
pub static THEME: GlobalSignal<String> = Global::new(|| "cupcake".into());
