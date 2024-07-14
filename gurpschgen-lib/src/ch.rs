use std::collections::HashMap;

use crate::{attrib::{Attribute, AttributeType}, gender::Gender};

/**
 Character container.
 */
pub struct Ch {
    pub name: String,
    pub attrib: HashMap<AttributeType, Attribute>,
    pub gender: Option<Gender>,
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
            gender: None,// will be chosen later.
        }
    }
}
