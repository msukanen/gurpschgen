/**
 A trait for anything with name.
 */
pub trait HasName {
    /**
     Get the name of... something or other ;-)

     **Returns** a name, obviously.
     */
    fn name(&self) -> &str;
}
