use dioxus::prelude::*;

use crate::Route;

#[component]
pub(crate) fn Help(id: i32) -> Element {
    rsx! {
        Link { to: Route::Main {}, "Go back" }
        div {
            h1 { "Help post {id}" }
        }
    }
}
