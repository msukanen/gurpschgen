#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info};
use gurpschgen_lib::dta::{genre::list_genre_files, locate_dta::locate_dta};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
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
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        div {
            h1 { "Blog post {id}" }
        }
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        Link {
            to: Route::ChooseGenre {  },
            "Go choose genre"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}

#[component]
fn ChooseGenre() -> Element {
    let mut genre = use_signal(|| "".to_string());
    let genre_list = list_genre_files();

    rsx! {
        div {
            h1 { "Choose a genre below:" }
            for (c,g) in genre_list.iter().enumerate() {
                div { "{c}: {g.display()}" }
            }
        }
    }
}
