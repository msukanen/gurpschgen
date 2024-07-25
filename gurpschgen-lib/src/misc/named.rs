/**
 A trait for anything with name.
 */
pub trait Named {
    /**
     Get the name of... something or other ;-)

     **Returns** a name, obviously.
     */
    fn name(&self) -> &str;
}
