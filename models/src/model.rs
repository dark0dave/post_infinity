use std::rc::Rc;

use std::fmt::Debug;

use erased_serde::Serialize;

use crate::tlk::Lookup;

pub trait Model: Debug + Serialize {
    fn create_as_rc(buffer: &[u8]) -> Rc<dyn Model>
    where
        Self: Sized;
    fn name(&self, lookup: &Lookup) -> String;
    fn new(buffer: &[u8]) -> Self
    where
        Self: Sized;
    fn to_bytes(&self) -> Vec<u8>;
}
