use std::rc::Rc;

use std::fmt::Debug;

use erased_serde::Serialize;

pub trait Model: Debug + Serialize {
    fn new(buffer: &[u8]) -> Self
    where
        Self: Sized;
    fn create_as_box(buffer: &[u8]) -> Rc<dyn Model>
    where
        Self: Sized;
}
