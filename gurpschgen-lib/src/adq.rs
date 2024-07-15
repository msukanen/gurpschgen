pub mod quirk;

use crate::misc::costly::Costly;

pub trait Adq: Costly {
    /**
     Name of something or other, obviously.
     */
    fn name(&self) -> &str;
}
