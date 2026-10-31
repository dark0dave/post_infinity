use std::fmt::Display;

use serde::{Deserialize, Serialize};
use zerocopy_derive::IntoBytes;

#[derive(Debug, PartialEq, Serialize, Deserialize, IntoBytes)]
#[repr(C, packed)]
pub struct ZeroCharVec<'data> {
    #[serde(borrow, flatten)]
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
        let str = std::str::from_utf8(self.inner).map_err(|err| {
            log::error!("{err}");
            std::fmt::Error
        })?;
        f.write_str(str)
    }
}
