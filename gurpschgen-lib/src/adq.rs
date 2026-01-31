//! Advantages, Disadvantages and Quirks — [Adq].
use serde::{Deserialize, Serialize};

use crate::{misc::{costly::HasCost, leveled::HasLevel, mod_grouped::HasModGroups, named::HasName}};

/// Container for advantages, disadvantages and quirks.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Adq {
    pub name: String,
    pub initial_cost: i32,
    pub cost_increment: i32,
    pub level: usize,
    pub max_level: usize,
    pub bonus_mods: Vec<String>,
    pub given: Vec<String>,
    pub mod_groups: Vec<String>,
}

impl Adq {
    /// Get initial purchasing point cost (a.k.a. cost of the 1st level/rank).
    pub fn base_cost(&self) -> i32 {
        self.initial_cost
    }

    /// Get past-1st-level cost increment; applied after 1st level/rank for each additional level from there on.
    pub fn next_cost(&self) -> i32 {
        self.cost_increment
    }

    /// Get names of what the [Adq] gives with it, if anything.
    pub fn gives(&self) -> &Vec<String> {
        &self.given
    }

    /// Whatever these are… Nobody Knows™
    pub fn bonus_mods(&self) -> &Vec<String> {
        &self.bonus_mods
    }
}

impl HasName for Adq {
    fn name(&self) -> &str {
        &self.name
    }
}

impl HasCost for Adq {
    fn cost(&self) -> f64 {
        //TODO: initial establishment of cost calc.
        (match self.level {
            ..=0 => 0,
            1 => self.initial_cost,
            n => self.initial_cost + (n as i32 - 1) * self.cost_increment
        }) as f64
    }
}

impl HasLevel for Adq {
    fn level(&self) -> usize {
        self.level
    }

    fn max_level(&self) -> Option<usize> {
        self.max_level.into()
    }
}

impl HasModGroups for Adq {
    fn mod_groups(&self) -> &Vec<String> {
        &self.mod_groups
    }
}
