use std::path::PathBuf;

use dioxus::prelude::*;
use gurpschgen_lib::dta::genre::Genre;

#[component]
pub(crate) fn CharacterSheet(genre: String) -> Element {
    let mut val_st = use_signal(|| 10);
    let mut val_dx = use_signal(|| 10);
    let mut val_iq = use_signal(|| 10);
    let mut val_ht = use_signal(|| 10);
    let g = Genre::load(&PathBuf::from(genre.to_string()));

    rsx! {
        div {
            "TODO: a character sheet, using {g.title}"
        }

        div {
            id: "attr_st",
            "ST",
            input {
                id: "attr_st_val",
                name: "attr_st_val",
                r#type: "number",
                //min: "1",// redundant
                //max: "20",// redundant
                value: val_st.to_string(),
                readonly: "true",// no manual entry allowed
                width: "3",// -???
            },
            span {
                button { onclick: move |_| if val_st() < 20 { val_st += 1 }, "↑" }
                button { onclick: move |_| if val_st() >  1 { val_st -= 1 }, "↓" }
            }
        }

        div {
            id: "attr_dx",
            "DX",
            input {
                id: "attr_dx_val",
                name: "attr_dx_val",
                r#type: "number",
                //min: "1",// redundant
                //max: "20",// redundant
                value: val_dx.to_string(),
                readonly: "true",// no manual entry allowed
                width: "3",// -???
            },
            span {
                button { onclick: move |_| if val_dx() < 20 { val_dx += 1 }, "↑" }
                button { onclick: move |_| if val_dx() >  1 { val_dx -= 1 }, "↓" }
            }
        }

        div {
            id: "attr_iq",
            "IQ",
            input {
                id: "attr_iq_val",
                name: "attr_iq_val",
                r#type: "number",
                //min: "1",// redundant
                //max: "20",// redundant
                value: val_st.to_string(),
                readonly: "true",// no manual entry allowed
                width: "3",// -???
            },
            span {
                button { onclick: move |_| if val_st() < 20 { val_st += 1 }, "↑" }
                button { onclick: move |_| if val_st() >  1 { val_st -= 1 }, "↓" }
            }
        }

        div {
            id: "attr_ht",
            "HT",
        }
    }
}

