use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::{common::variable_char_array::VariableCharArray, model::Model, tlk::Lookup};

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Biography(pub VariableCharArray);

impl Model for Biography {
    fn new(buffer: &[u8]) -> Self {
        Self(VariableCharArray(buffer.into()))
    }

    fn create_as_rc(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }

    fn name(&self, _lookup: &Lookup) -> String {
        todo!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0 .0.to_vec()
    }
}
