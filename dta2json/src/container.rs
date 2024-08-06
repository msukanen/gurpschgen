use gurpschgen_lib::equipment::item::container::Container;
use regex::Captures;

pub(crate) fn container_from_captures(value: Captures<'_>) -> Container {
    if let Some(wt) = value.name("lbs") {
        Container::Wt(wt.as_str().parse::<i32>().unwrap())
    } else {
        todo!("Container::from: \"{:?}\"", value)
    }
}
