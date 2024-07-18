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
            n => panic!("FATAL: unknown 'type' \"{n}\" detected!")
        }
    }
}
