pub trait Noted {
    fn notes(&self) -> Option<&str>;
}
