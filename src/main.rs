#![cfg_attr(feature = "bundle", windows_subsystem = "windows")]

use app::App;
// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;

use store::use_init_language;

// use components::Hero;
// use components::Welcome;

mod app;
/// Define a components module that contains all shared components for our app.
mod components;
mod store;
/// Define a views module that contains the UI for all Layouts and Routes for our app.
mod views;

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
// The asset macro also minifies some assets like CSS and JS to make bundled smaller
const MAIN_CSS: Asset = asset!("/assets/styling/main.scss");

const NORMALIZE_CSS: Asset = asset!("/assets/modern-normalize.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // Init logger
    // dioxus_logger::init(Level::INFO).expect("failed to init logger");
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(Main);
}

// Globals are created the first time you access them with the closure you pass to Global::new
// static THEME: GlobalSignal<String> = Global::new(|| "cupcake".into());

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn Main() -> Element {
    use_init_language();

    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        // In addition to element and text (which we will see later), rsx can contain other components. In this case,
        // we are using the `document::Link` component to add a link to our favicon and main CSS file into the head of our app.
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: NORMALIZE_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: MAIN_CSS }


        App {}
    }
}
