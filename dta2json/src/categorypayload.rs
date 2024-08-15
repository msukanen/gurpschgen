use gurpschgen_lib::context::{CategoryPayload, Context};

use crate::{adq::adq_from_tuple, equipment::equipment_from_tuple, skill::{skill_from_tuple, RX_SIMPLE}};

pub(crate) fn category_payload_from_triple(value: (&Context, &str, &str)) -> CategoryPayload {
    match value.0 {
        Context::Advantage |
        Context::Package   => CategoryPayload::Advantage(adq_from_tuple((value.1, value.2))),
        Context::Disadvantage => CategoryPayload::Disadvantage(adq_from_tuple((value.1, value.2))),
        Context::Quirk => {
            if let Some(cap) = RX_SIMPLE.captures(value.1) {
                CategoryPayload::Quirk(cap.name("anything").unwrap().as_str().to_string())
            } else {
                panic!("FATAL: malformed QUIRK \"{}\"", value.1)
            }
        },
        Context::Equipment => CategoryPayload::Equipment(equipment_from_tuple((value.1, value.2))),
        Context::Genre => CategoryPayload::Genre(genre_from_tuple((value.2))),
        Context::Bonus => CategoryPayload::Bonus(value.1.to_string()),
        Context::Modifier => CategoryPayload::Modifier(value.1.to_string()),
        Context::Skill |
        Context::Spell => CategoryPayload::Skill(skill_from_tuple((value.1, value.2))),
        Context::Counter => CategoryPayload::Counter(value.1.to_string()),
    }
}
