use serde::{Deserialize, Serialize};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum AttributeType {
    DX, HT, IQ, ST,
}

impl From<&str> for AttributeType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "dx" => Self::DX,
            "ht" => Self::HT,
            "iq" => Self::IQ,
            "st" => Self::ST,
            unk => panic!("There is no such AttributeType as '{unk}'!")
        }
    }
}
