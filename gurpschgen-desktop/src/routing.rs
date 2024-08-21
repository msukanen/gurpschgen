use dioxus::prelude::*;

use crate::root::Root;
use crate::help::Help;
use crate::genre::ChooseGenre;
use crate::chsheet::CharacterSheet;

#[derive(Clone, Routable, Debug, PartialEq)]
pub(crate) enum Route {
    #[route("/")]
    Root {},
    #[route("/help/:id")]
    Help { id: i32 },
    #[route("/genre")]
    ChooseGenre {},
    #[route("/sheet/:genre")]
    CharacterSheet { genre: String },
}
