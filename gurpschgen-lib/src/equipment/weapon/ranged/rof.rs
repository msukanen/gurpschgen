use regex::Captures;

#[derive(Debug, Clone)]
pub enum RoF {
    Auto(i32),
    Semi(i32),
    Slow(i32, i32),
}

impl From<Captures<'_>> for RoF {
    fn from(value: Captures<'_>) -> Self {
        let x = value.name("rof").unwrap().as_str();
        let n = value.name("rof1").unwrap().as_str().parse::<i32>().unwrap();
        if x.contains("~") {
            Self::Semi(n)
        } else if x.contains("/") {
            Self::Slow(n, value.name("rof2").unwrap().as_str().parse::<i32>().unwrap())
        } else {
            Self::Auto(n)
        }
    }
}
