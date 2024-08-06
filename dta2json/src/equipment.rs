use gurpschgen_lib::equipment::Equipment;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{armor::{armor_from_tuple, RX_IS_ARMOR}, item::item_from_tuple, weapon::{wpn_from_tuple, RX_SIMPLE_ANY_WPN}};

pub(crate) static RX_TL: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:TL\s*(?<tl>\d+))").unwrap());
pub(crate) static RX_COUNTRY: Lazy<Regex> = Lazy::new(||Regex::new(r"US(SR)?|BE|GE|IT|IS|GR|UK|FI|SE|NO|INT").unwrap());
pub(crate) static RX_LEGALITY: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:LC\s*(?<lc>\d+))").unwrap());

pub(crate) fn equipment_from_tuple(value: (&str, &str)) -> Equipment {
    // it's an armor?
    if let Some(_) = RX_IS_ARMOR.captures(value.1) {
        Equipment::Armor(armor_from_tuple(value))
    }
    // it's a weapon?
    else if let Some(_) = RX_SIMPLE_ANY_WPN.captures(value.1) {
        Equipment::Weapon(wpn_from_tuple(value))
    }
    // nah... not armor or weapon, something else.
    else {
        Equipment::Item(item_from_tuple(value))
    }
}
