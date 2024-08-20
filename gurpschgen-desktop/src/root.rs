use dioxus::prelude::*;

use crate::Route;

#[component]
pub(crate) fn Root() -> Element {
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
