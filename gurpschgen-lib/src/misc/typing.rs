use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::context::Context;

use super::category::Category;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContextPayload {
    pub context: Context,
    pub items: HashMap<String, Category>,
}

impl ContextPayload {
    pub fn new(context: Context) -> Self {
        ContextPayload { context, items: HashMap::new() }
    }
}
