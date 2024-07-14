use std::collections::HashMap;

use crate::attrib::{Attribute, AttributeType};

/**
 Character container.
 */
pub struct Ch {
    pub name: String,
    pub attrib: HashMap<AttributeType, Attribute>,
}

impl Ch {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            attrib: {
                let mut m = HashMap::new();
                m.insert(AttributeType::DX, Attribute::default(AttributeType::DX));
                m.insert(AttributeType::HT, Attribute::default(AttributeType::HT));
                m.insert(AttributeType::IQ, Attribute::default(AttributeType::IQ));
                m.insert(AttributeType::ST, Attribute::default(AttributeType::ST));
                m
            },
        }
    }
}
