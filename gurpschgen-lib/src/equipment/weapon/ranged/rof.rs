use serde::{Deserialize, Serialize};

/**
 Rate of Fire (RoF).
 */
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum RoF {
    /// **X*** → auto &ndash; e.g. SMGs, LMGs, etc.
    FullAuto(i32),
    /// **X~** → semi-auto &ndash; e.g. Colt 1911
    SemiAuto(i32),
    /// **Skill/X** → RoF based on skill's divisor.
    Skill(i32),
    /// **1/X** → multiple seconds to reload &ndash; blunderbus, etc.
    Slow(i32, i32),
    /// **X** → 6-shooters, etc.
    Trigger(i32),
}
