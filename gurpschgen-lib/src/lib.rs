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
    // notes, cost, wt, skill, modgr
    static RX_ITEM: Regex = Regex::new(r"^\s*(?<notes>[^;]*)?(;\s*((?<cost>\d+([.]?\d+)?)(\s*,\s*(?<wt>\d+([.]?\d+)?))?(;\s*((?<skill>[^;]*)?(;\s*((?:[^;]*)?(;\s*(?<modgr>[^;]*)?)?)?)?)?)?)?)?").unwrap();
    // (c1, c2)|c3, maxlvl, bonus, given, modgr
    static RX_ADQ: Regex = Regex::new(r"^\s*((?<c1>\d+)\s*/\s*(?<c2>\d+)|(?<c3>\d+))(\s*;\s*((?<maxlvl>\d+)?(\s*;\s*((?<bonus>[^;]*)(\s*;\s*((?<given>[^;]*)(;\s*(?<modgr>[^;]*)?)?)?)?)?)?)?)?").unwrap();
    static RX_SIMPLE_RANGED: Regex = Regex::new(r"(?:SS\s*\d)").unwrap();
    // cost, wt
    static RX_COST_WEIGHT: Regex = Regex::new(r"(?:\s*(?<cost>\d+(?:[.]\d+)?)(?:\s*,\s*(?<wt>\d+(?:[.]\d+)?))?)").unwrap();
    // dtype, (ddel (, dmod?)) | (dd (, ddm? (, dmul?)))
    static RX_DMGD: Regex = Regex::new(r"(?:\s*(?<dtype>Cut|Cr|Imp)\/((?:(?:(?<ddel>Sw|Thr)(?<dmod>[+-]\d+)?))|(?:(?<dd>\d+)(?<maybed>d?)(?:(?<ddm>[-+]\d+)(?:\([xX](?<dmul>\d+(?:[.]\d+)?)\))?)?)))").unwrap();
}
