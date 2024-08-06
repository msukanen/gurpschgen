use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Container {
    Wt(i32),
    Liquid(i32),
}
