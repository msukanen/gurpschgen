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

thread_local!(static RX_ITEM: Regex = Regex::new(r"^\s*(?<notes>[^;]*)?(;\s*((?<cost>\d+([.]?\d+)?)(\s*,\s*(?<wt>\d+([.]?\d+)?))?(;\s*((?<skill>[^;]*)?(;\s*((?:[^;]*)?(;\s*(?<modgr>[^;]*)?)?)?)?)?)?)?)?").unwrap());
thread_local!(static RX_ADQ: Regex = Regex::new(r"^\s*((?<c1>\d+)\s*/\s*(?<c2>\d+)|(?<c3>\d+))(\s*;\s*((?<maxlvl>\d+)?(\s*;\s*((?<bonus>[^;]*)(\s*;\s*((?<given>[^;]*)(;\s*(?<modgr>[^;]*)?)?)?)?)?)?)?)?").unwrap());
thread_local!(static RX_ARMOR: Regex = Regex::new(r"^\s*((?<shield>(PD\s*\d+\s*;))|((?<armor>(PD\s*(?<pd>\d+)\s*,\s*DR\s*(?<dr>\d+)\s*,\s*Covers:\s*(?<cover>[-, 0-9]+))\s*;)))?(\s*((?<cost>\d+([.]?\d+)?)(\s*,\s*(?<wt>\d+([.]?\d+)?))?\s*(;\s*((?:[^;]*)?(;\s*((?:[^;]*)?(;\s*(?<modgr>[^;]*)?)?)?)?)?)?)?)?").unwrap());
