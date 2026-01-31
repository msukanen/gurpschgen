//! Container "types" dwell here.
use serde::{Deserialize, Serialize};

/// Container type differentiation enum.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Container {
    /// Weight-based containers.
    /// 
    /// Payload `i32` *often* represents "lbs." but exact details depend on the whims of the GM…
    Wt(i32),
    /// Containers rated by amount of liquid they hold.
    /// 
    /// Payload `i32` *often* represents "US-gallons" but exact details depend on the whims of the GM…
    Liquid(i32),
}
