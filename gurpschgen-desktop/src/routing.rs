use dioxus::prelude::*;

use crate::root::Root;
use crate::help::Help;
use crate::genre::ChooseGenre;

#[derive(Clone, Routable, Debug, PartialEq)]
pub(crate) enum Route {
    #[route("/")]
    Root {},
    #[route("/help/:id")]
    Help { id: i32 },
    #[route("/genre")]
    ChooseGenre {},
}
