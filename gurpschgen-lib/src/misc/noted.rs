/**
 A trait for anything with "notes".
 */
pub trait Noted {
    /**
     Get associated note(s), if any.
     */
    fn notes(&self) -> Option<&str>;
}
