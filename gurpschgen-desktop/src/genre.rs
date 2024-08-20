use dioxus::prelude::*;
use gurpschgen_lib::dta::genre::list_genre_files;

use crate::Route;

#[component]
pub(crate) fn ChooseGenre() -> Element {
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
                    to: Route::Main{},
                    button { "Button" }
                }
            }
        }
    }
}
