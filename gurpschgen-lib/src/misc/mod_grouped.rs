/**
 A trait for anything with modifier groups.
 */
pub trait ModGrouped {
    /**
     Get the modifier groups that can be applied.
     */
    fn mod_groups(&self) -> &Vec<String>;
}
