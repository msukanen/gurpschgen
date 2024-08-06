use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::context::Context;

use super::category::Category;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Type {
    pub context: Context,
    pub items: HashMap<String, Category>,
}

impl Type {
    pub fn new(context: Context) -> Self {
        Type { context, items: HashMap::new() }
    }
}
