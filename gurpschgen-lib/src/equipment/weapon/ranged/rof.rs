use serde::{Deserialize, Serialize};

/**
 Rate of Fire (RoF).
 */
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum RoF {
    /// `X*` → auto (e.g. SMGs, LMGs, etc.).
    FullAuto(i32),
    /// `X~` → semi-auto (e.g. Colt 1911).
    SemiAuto(i32),
    /// `Skill/X` → RoF based on skill. One shot per `skill/X` seconds.
    Skill(i32),
    /// `1/X` → `X` seconds to reload (blunderbus, etc.).
    Slow(i32, i32),
    /// `X` → 6-shooters, etc.
    Trigger(i32),
}
