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
    //let mut count = use_signal(|| 0);

    rsx! {
        h1 { "gurpschgen-desktop" }
        div {
            "Note: this is a 3rd party tool that has no association whatsoever with Steve Jackson Games."
        }
        div {
            "To begin creating a character, "
            Link { to: Route::ChooseGenre {}, "choose a genre" }
            " you want/need to use."
        }
        /*
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
        */
    }
}

#[component]
fn ChooseGenre() -> Element {
    let mut genre: Signal<String> = use_signal(|| "".to_string());
    let genre_list = list_genre_files();

    rsx! {
        div {
            if genre.to_string().is_empty() {
                h1 { "Choose a genre below:" }
            } else {
                h1 { "Choose a genre below (current: {genre.to_string()})"}
            }
            for g in genre_list.into_iter() {
                button { onclick: move |_| genre.set(g.clone().display().to_string()), "{g.display()}" }
            }
        }
        if !genre.to_string().is_empty() {
            div {
                Link {
                    to: Route::Home{},
                    button { "Button" }
                }
            }
        }
    }
}
