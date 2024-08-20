use dioxus::prelude::*;

use crate::routing::Route;

/**
 TODO: help system.
 */
#[component]
pub(crate) fn Help(id: i32) -> Element {
    rsx! {
        Link { to: Route::Root {}, "Go back" }
        div {
            h1 { "Help post {id}" }
        }
    }
}
