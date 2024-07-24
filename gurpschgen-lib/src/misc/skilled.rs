/**
 A trait for anything with req.skill payload.
 */
pub trait Skilled {
    /**
     Get the associated req.skill payload, if any.
     */
    fn skill(&self) -> Option<&str>;
}
