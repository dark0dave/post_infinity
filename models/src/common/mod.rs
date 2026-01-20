use char_array::CharArray;
use serde::{
    Deserialize, Serialize,
    de::{Error, Visitor},
};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};
use zerovec::{make_ule, ule::ULE};

pub mod char_array;
pub mod feature_block;
pub mod header;
pub mod parsers;
pub mod strref;
pub mod types;

pub type Resref = CharArray<8>;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromBytes,
    IntoBytes,
    Immutable,
    KnownLayout,
    ULE,
)]
#[repr(C, packed)]
pub struct ZeroCharArray(pub(crate) [u8; 4]);

struct ZeroCharArrayVistor {}

impl Visitor<'_> for ZeroCharArrayVistor {
    type Value = ZeroCharArray;

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(ZeroCharArray(v.as_bytes().try_into().unwrap()))
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }
}

impl<'de> Deserialize<'de> for ZeroCharArray {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ZeroCharArrayVistor {})
    }
}

impl Serialize for ZeroCharArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(str::from_utf8({ self.0 }.as_slice()).unwrap_or_default())
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, FromBytes, IntoBytes, Immutable, KnownLayout,
)]
#[make_ule(ZeroResrefULE)]
#[repr(C, packed)]
pub struct ZeroResref([u8; 8]);

struct ZeroResrefVistor {}

impl Visitor<'_> for ZeroResrefVistor {
    type Value = ZeroResref;

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(ZeroResref(v.as_bytes().try_into().unwrap()))
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string")
    }
}

impl<'de> Deserialize<'de> for ZeroResref {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ZeroResrefVistor {})
    }
}

impl Serialize for ZeroResref {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(str::from_utf8({ self.0 }.as_slice()).unwrap_or_default())
    }
}
