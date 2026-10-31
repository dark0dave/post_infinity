use std::{
    convert::Infallible,
    fmt::{Debug, Display, Write},
};

use serde::{
    Deserialize, Serialize,
    de::{Error, Visitor},
};
use zerocopy::{AlignmentError, ConvertError, FromBytes, SizeError};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

#[derive(Debug, Clone, Copy, PartialEq, FromBytes, IntoBytes, Immutable, KnownLayout)]
#[repr(C)]
pub struct ZeroCharArray<const N: usize>([u8; N]);

impl<const N: usize> From<[u8; N]> for ZeroCharArray<N> {
    fn from(value: [u8; N]) -> Self {
        Self(value)
    }
}

impl<'data, const N: usize> TryFrom<&'data [u8]> for &'data ZeroCharArray<N> {
    type Error = ConvertError<
        AlignmentError<&'data [u8], ZeroCharArray<N>>,
        SizeError<&'data [u8], ZeroCharArray<N>>,
        Infallible,
    >;

    fn try_from(value: &'data [u8]) -> Result<Self, Self::Error> {
        <ZeroCharArray<N>>::ref_from_bytes(value)
    }
}

impl<'data, const N: usize> TryFrom<&'data str> for &'data ZeroCharArray<N> {
    type Error = ConvertError<
        AlignmentError<&'data [u8], ZeroCharArray<N>>,
        SizeError<&'data [u8], ZeroCharArray<N>>,
        Infallible,
    >;

    fn try_from(value: &'data str) -> Result<Self, Self::Error> {
        <ZeroCharArray<N>>::ref_from_bytes(value.as_bytes())
    }
}

impl<const N: usize> TryFrom<&'static [u8]> for ZeroCharArray<N> {
    type Error = SizeError<&'static [u8], Self>;

    fn try_from(value: &'static [u8]) -> Result<Self, Self::Error> {
        <ZeroCharArray<N>>::read_from_bytes(value)
    }
}

impl<const N: usize> TryFrom<&'static str> for ZeroCharArray<N> {
    type Error = SizeError<&'static [u8], Self>;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        <ZeroCharArray<N>>::read_from_bytes(value.as_bytes())
    }
}

impl<const N: usize> Display for ZeroCharArray<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for element in self.0 {
            f.write_char(element as char)?;
        }
        Ok(())
    }
}

impl<const N: usize> Serialize for ZeroCharArray<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct ZeroCharArrayVistor<const N: usize> {}

impl<const N: usize> Visitor<'_> for ZeroCharArrayVistor<N> {
    type Value = ZeroCharArray<N>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }
    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(ZeroCharArray(
            v.as_bytes()
                .try_into()
                .map_err(|_| E::custom("Failed to parse"))?,
        ))
    }
}

impl<'de, const N: usize> Deserialize<'de> for ZeroCharArray<N> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_string(ZeroCharArrayVistor {})
    }
}
