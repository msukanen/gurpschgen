use gurpschgen_lib::context::Context;

pub(crate) fn context_from_str(value: &str) -> Context{
    match value {
        "advantage" => Context::Advantage,
        "bonus" => Context::Bonus,
        "counter" => Context::Counter,
        "disadvantage" => Context::Disadvantage,
        "equipment" => Context::Equipment,
        "modifier" => Context::Modifier,
        "package" => Context::Package,
        "quirk" => Context::Quirk,
        "skill" => Context::Skill,
        "spell" => Context::Spell,
        n => panic!("FATAL: unknown 'type' \"{n}\" detected!")
    }
}
