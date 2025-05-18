use crate::components::Language;
use crate::Route;
use dioxus::{logger::tracing, prelude::*};
use dioxus_free_icons::icons::hi_outline_icons::HiBell;
use dioxus_free_icons::Icon;

const NAVBAR_CSS: Asset = asset!("/assets/styling/views/navbar.scss");
const AVATAR_IMG: Asset = asset!("/assets/ren_dislikes_sweets.png");

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
///
///
/// This layout component wraps the UI of [Route::Home] and [Route::Blog] in a common navbar. The contents of the Home and Blog
/// routes will be rendered under the outlet inside this component
#[component]
pub fn Navbar() -> Element {


    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        header {
            id: "navbar",
            class: "ui-navbar bg-base-100 shadow-sm sticky top-0",

            div { class: "flex-1",
                span { class: "text-xl px-4", "Leact" }
            }
            div { class: "flex-none flex gap-x-1",

                button { class: "ui-btn ui-btn-ghost ui-btn-circle",
                    div { class: "ui-indicator",
                        Icon { class: "icon-hi-o", icon: HiBell }
                        span { class: "ui-badge ui-badge-xs ui-badge-primary ui-indicator-item" }
                    }
                }
                div { class: "ui-dropdown ui-dropdown-center",
                    div {
                        role: "button",
                        tabindex: "0",
                        class: "ui-btn ui-btn-ghost ui-btn-circle ui-avatar",
                        div { class: "w-10 rounded-full",
                            img {
                                src: AVATAR_IMG,
                                alt: "Tailwind CSS Navbar component",
                            }
                        }
                    }
                    ul {
                        tabindex: "0",
                        class: "ui-menu ui-dropdown-content bg-base-100 rounded-box z-1 mt-3  p-2 shadow",
                        li {
                            a { class: "justify-between",
                                "Profile"
                                span { class: "ui-badge ui-badge-sm", "New" }
                            }
                        }
                        li {
                            a { "Settings" }
                        }
                        li {
                            a { "Logout" }
                        }
                    }
                }
                Language {}
            }
        }

        // The `Outlet` component is used to render the next component inside the layout. In this case, it will render either
        // the [`Home`] or [`Blog`] component depending on the current route.
        Outlet::<Route> {}
    }
}
