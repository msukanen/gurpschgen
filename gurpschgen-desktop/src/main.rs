#![allow(non_snake_case)]

mod root;
mod help;
mod genre;
mod routing;
mod chsheet;

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info};
use gurpschgen_lib::dta::locate_dta::locate_dta;
use routing::Route;

/**
 The main root of all evil...
 */
fn main() {
    locate_dta(true);
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    
    dioxus::launch(App);
}

/**
 The app launcher element... Preps routing, and that's about that.
 */
#[component]
pub(crate) fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
