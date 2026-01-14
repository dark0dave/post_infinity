use std::{
    cmp::min,
    fmt::{Debug, Display, Write},
    str,
};

use binrw::{BinRead, BinWrite};
use serde::{
    Deserialize, Serialize,
    de::{Error, Visitor},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, BinRead, BinWrite)]
pub struct CharArray<const N: usize>(pub(crate) [u8; N]);

impl<const N: usize> Serialize for CharArray<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct CharArrayVistor<const N: usize> {}

impl<const N: usize> Visitor<'_> for CharArrayVistor<N> {
    type Value = CharArray<N>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }
    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(CharArray::<N>::from(v))
    }
}

impl<'de, const N: usize> Deserialize<'de> for CharArray<N> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_string(CharArrayVistor {})
    }
}

impl<const N: usize> Display for CharArray<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for element in self.0 {
            f.write_char(char::from(element))?;
        }
        Ok(())
    }
}

impl<const N: usize> From<&[u8]> for CharArray<N> {
    fn from(value: &[u8]) -> Self {
        let mut out = [0; N];
        unsafe {
            std::ptr::copy_nonoverlapping(value.as_ptr(), out.as_mut_ptr(), min(N, value.len()));
        }
        if value.len() > N {
            log::warn!("truncating {value:?} to {out:?}");
        }
        Self(out)
    }
}

impl<const N: usize> From<&str> for CharArray<N> {
    fn from(value: &str) -> Self {
        value.as_bytes().into()
    }
}

impl<const N: usize> From<String> for CharArray<N> {
    fn from(value: String) -> Self {
        value.as_bytes().into()
    }
}
