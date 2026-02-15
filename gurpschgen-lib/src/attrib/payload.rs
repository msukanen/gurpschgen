use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::modifier::{Modifier, ModifierValue};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttributePayload {
    pub(super) modifiers: HashMap<Modifier, Option<ModifierValue>>,
}

impl Default for AttributePayload {
    fn default() -> Self {
        Self { modifiers: HashMap::new() }
    }
}
