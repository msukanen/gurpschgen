use crate::{adq::Adq, equipment::Equipment, skill::Skill, RX_SIMPLE};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Context {
    Advantage,
    Bonus,
    Counter,
    Disadvantage,
    Equipment,
    Modifier,
    Package,
    Quirk,
    Skill,
    Spell,
}

#[derive(Debug, Clone)]
pub enum CategoryPayload {
    Advantage(Adq),
    Bonus(String),
    Counter(String),
    Disadvantage(Adq),
    Equipment(Equipment),
    Modifier(String),
    Package(String),
    Quirk(String),
    Skill(Skill),
    Spell(String),
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Advantage => "advantage",
            Self::Bonus => "bonus",
            Self::Counter => "counter",
            Self::Disadvantage => "disadvantage",
            Self::Equipment => "equipment",
            Self::Modifier => "modifier",
            Self::Package => "package",
            Self::Quirk => "quirk",
            Self::Skill => "skill",
            Self::Spell => "spell",
        })
    }
}

impl From<&str> for Context {
    fn from(value: &str) -> Self {
        match value {
            "advantage" => Self::Advantage,
            "bonus" => Self::Bonus,
            "counter" => Self::Counter,
            "disadvantage" => Self::Disadvantage,
            "equipment" => Self::Equipment,
            "modifier" => Self::Modifier,
            "package" => Self::Package,
            "quirk" => Self::Quirk,
            "skill" => Self::Skill,
            "spell" => Self::Spell,
            n => panic!("FATAL: unknown 'type' \"{n}\" detected!")
        }
    }
}

impl From<(&Context, &str, &str)> for CategoryPayload {
    fn from(value: (&Context, &str, &str)) -> Self {
        match value.0 {
            Context::Advantage => Self::Advantage(Adq::from((value.1, value.2))),
            Context::Disadvantage => Self::Disadvantage(Adq::from((value.1, value.2))),
            Context::Quirk => {
                if let Some(cap) = RX_SIMPLE.captures(value.1) {
                    Self::Quirk(cap.name("anything").unwrap().as_str().to_string())
                } else {
                    panic!("FATAL: malformed QUIRK \"{}\"", value.1)
                }
            },
            Context::Equipment => Self::Equipment(Equipment::from((value.1, value.2))),
            Context::Bonus => Self::Bonus(value.1.to_string()),
            Context::Modifier => Self::Modifier(value.1.to_string()),
            Context::Skill => Self::Skill(Skill::from((value.1, value.2))),
            _ => {
                if let Some(cap) = RX_SIMPLE.captures(value.1) {
                    Self::Quirk(cap.name("anything").unwrap().as_str().to_string())
                } else {
                    panic!("FATAL: malformed QUIRK \"{}\"", value.1)
                }
            }
        }
    }
}
