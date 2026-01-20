use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::model::Parseable;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Tileset<'data> {
    #[serde(borrow)]
    pub data: &'data [u8],
}

impl<'data> Parseable<'data> for Tileset<'data> {}

impl<'data> TryFrom<&'data [u8]> for Tileset<'data> {
    type Error = Box<dyn Error>;

    fn try_from(value: &'data [u8]) -> Result<Self, Self::Error> {
        Ok(Self { data: value })
    }
}

impl<'data> TryInto<Vec<u8>> for Tileset<'data> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Ok(self.data.to_vec())
    }
}
