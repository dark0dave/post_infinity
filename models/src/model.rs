use std::rc::Rc;

use std::fmt::Debug;

pub trait Model
where
    Self: Debug,
{
    fn new(buffer: &[u8]) -> Self
    where
        Self: Sized;
    fn create_as_rc(buffer: &[u8]) -> Rc<dyn Model>
    where
        Self: Sized;
}
