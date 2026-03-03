//! Runtime assigned ID's…

use std::hash::{DefaultHasher, Hash, Hasher};

use crate::misc::named::HasName;

pub type RuntimeID = usize;

/// A trait for anything that has an associated runtime/temporary ID value.
pub trait HasRuntimeID {
    /// Get runtime ID.
    fn runtime_id(&self) -> usize where Self: HasName {
        self.name().runtime_id()
    }
}

pub(crate) trait StringHash {
    fn runtime_id(&self) -> RuntimeID;
}

impl StringHash for &str {
    fn runtime_id(&self) -> RuntimeID {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish() as RuntimeID
    }
}
