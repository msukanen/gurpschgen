use crate::{adq::Adq, equipment::Equipment, RX_SIMPLE};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Context {
    Advantage,
    Bonus,
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
    Disadvantage(Adq),
    Equipment(Equipment),
    Modifier(String),
    Package(String),
    Quirk(String),
    Skill(String),
    Spell(String),
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Advantage => "advantage",
            Self::Bonus => "bonus",
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
                if let Some(cap) = RX_SIMPLE.with(|rx| rx.captures(value.1)) {
                    Self::Quirk(cap.name("anything").unwrap().as_str().to_string())
                } else {
                    panic!("FATAL: malformed QUIRK \"{}\"", value.1)
                }
            },
            Context::Equipment => Self::Equipment(Equipment::from((value.1, value.2))),
            _ => {
                if let Some(cap) = RX_SIMPLE.with(|rx| rx.captures(value.1)) {
                    Self::Quirk(cap.name("anything").unwrap().as_str().to_string())
                } else {
                    panic!("FATAL: malformed QUIRK \"{}\"", value.1)
                }
            }
        }
    }
}
