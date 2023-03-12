use std::rc::Rc;

use serde::Serialize;

use crate::{common::varriable_char_array::VarriableCharArray, model::Model};

#[derive(Debug, Serialize)]
pub struct Biography(pub VarriableCharArray);

impl Model for Biography {
    fn new(buffer: &[u8]) -> Self {
        Self(VarriableCharArray(buffer.to_vec()))
    }

    fn create_as_box(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }
}
