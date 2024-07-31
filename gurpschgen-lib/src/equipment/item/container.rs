use regex::Captures;

#[derive(Debug, Clone)]
pub enum Container {
    Wt(i32),
    Liquid(i32),
}

impl From<Captures<'_>> for Container {
    fn from(value: Captures<'_>) -> Self {
        if let Some(wt) = value.name("lbs") {
            Self::Wt(wt.as_str().parse::<i32>().unwrap())
        } else {
            todo!("Container::from: \"{:?}\"", value)
        }
    }
}
