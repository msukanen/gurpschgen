//!
//! GURPS Character Generator data handler library.
//! 
use regex::Regex;

pub mod attrib;
pub mod edition;
pub mod config;
pub mod modifier;
pub mod ch;
pub mod misc;
pub mod gender;
pub mod adq;
pub mod dta;
pub mod context;
pub mod equipment;
pub mod damage;

thread_local! {
    static RX_COST_WEIGHT: Regex = Regex::new(r"(?:\s*(?<cost>\d+(?:[.]\d+)?)(?:\s*,\s*(?<wt>\d+(?:[.]\d+)?))?)").unwrap();
    static RX_SIMPLE: Regex = Regex::new(r"^(?:\s*(?<anything>[^;]+))").unwrap();
}
