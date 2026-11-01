use std::fmt::{Display, Write};

use serde::{Deserialize, Serialize};
use zerocopy_derive::IntoBytes;

#[derive(Debug, PartialEq, Deserialize, IntoBytes)]
#[repr(C, packed)]
pub struct ZeroCharVec<'data> {
    #[serde(borrow)]
    inner: &'data [u8],
}

impl<'data> From<&'data [u8]> for ZeroCharVec<'data> {
    fn from(value: &'data [u8]) -> Self {
        Self { inner: value }
    }
}

impl<'data> From<&'data str> for ZeroCharVec<'data> {
    fn from(value: &'data str) -> Self {
        Self {
            inner: value.as_bytes(),
        }
    }
}

impl<'data> Display for ZeroCharVec<'data> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for element in self.inner {
            f.write_char(*element as char)?;
        }
        Ok(())
    }
}

impl<'data> Serialize for ZeroCharVec<'data> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
