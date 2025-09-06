use std::{
    fmt::{Debug, Display, Write},
    str,
};

use binrw::{BinRead, BinWrite};
use serde::{
    de::{Error, Visitor},
    Deserialize, Serialize,
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
        if value.len() > N {
            let trunc = value[0..N].try_into().unwrap();
            log::warn!("truncating {value:?} to {trunc:?}");
            Self(trunc)
        } else {
            let mut out = [0; N];
            for (i, item) in value.iter().enumerate() {
                out[i] = *item;
            }
            Self(out)
        }
    }
}

impl<const N: usize> From<&str> for CharArray<N> {
    fn from(value: &str) -> Self {
        value.as_bytes().into()
    }
}

impl<const N: usize> From<CharArray<N>> for String {
    fn from(val: CharArray<N>) -> Self {
        String::from_utf8(val.0.to_vec()).unwrap_or_default()
    }
}

impl<const N: usize> From<String> for CharArray<N> {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}
