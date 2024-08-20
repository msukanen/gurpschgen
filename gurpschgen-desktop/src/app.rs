use dioxus::prelude::*;

use crate::Route;

#[component]
pub(crate) fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
