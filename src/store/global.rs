use dioxus::signals::{Global, GlobalSignal};

// Globals are created the first time you access them with the closure you pass to Global::new
pub static THEME: GlobalSignal<String> = Global::new(|| "cupcake".into());
