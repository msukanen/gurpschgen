use crate::misc::costly::Costly;

use super::Adq;

pub struct Quirk {
    name: String,
}

impl Adq for Quirk {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Costly for Quirk {
    fn cost(&self) -> f64 {
        -1.0 // Quirks cost a flat -1 points.
    }
}

impl Quirk {
    /**
     Instantiate a new [Quirk].
     */
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }

    /**
     Set `name` of the [Quirk].
     */
    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }
}

impl std::fmt::Display for Quirk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl From<&str> for Quirk {
    fn from(value: &str) -> Self {
        Self { name: value.to_string() }
    }
}

impl From<String> for Quirk {
    fn from(value: String) -> Self {
        Self { name: value }
    }
}

impl From<&String> for Quirk {
    fn from(value: &String) -> Self {
        Self { name: value.clone() }
    }
}
