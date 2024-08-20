#![allow(non_snake_case)]

mod app;
mod help;
mod genre;

use app::App;
use help::Help;
use genre::ChooseGenre;
use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info};
use gurpschgen_lib::dta::locate_dta::locate_dta;

#[derive(Clone, Routable, Debug, PartialEq)]
pub(crate) enum Route {
    #[route("/")]
    Main {},
    #[route("/help/:id")]
    Help { id: i32 },
    #[route("/genre")]
    ChooseGenre {},
}

fn main() {
    locate_dta(true);
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    
    dioxus::launch(App);
}

#[component]
fn Main() -> Element {
    //let mut count = use_signal(|| 0);

    rsx! {
        h1 { "gurpschgen-desktop" }
        div {
            "Note: this is a 3rd party tool that has " b{"no"}" association whatsoever with " em{"Steve Jackson Games"}"."
        }
        div {
            "To begin creating a character, go and "
            Link { to: Route::ChooseGenre {}, "choose a genre" }
            " you want/need to use."
        }
    }
}
