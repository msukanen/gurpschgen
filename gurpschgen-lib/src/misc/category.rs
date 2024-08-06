use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::context::CategoryPayload;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Category {
    pub name: String,
    pub items: HashMap<String, CategoryPayload>,
}

impl Category {
    pub fn new(name: &str) -> Self {
        Category { name: name.to_string(), items: HashMap::new() }
    }
}
