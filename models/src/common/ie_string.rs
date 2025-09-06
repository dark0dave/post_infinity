use std::{fmt::Debug, str};

use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IEString<'a>(#[serde(borrow)] &'a str);

impl<'a> BinWrite for IEString<'a> {
    type Args<'b> = &'a [u8];

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        todo!()
    }
}

impl<'a> BinRead for IEString<'a> {
    type Args<'b> = &'a [u8];

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        todo!()
    }
}

impl<'a> From<&'a [u8]> for IEString<'a> {
    fn from(value: &'a [u8]) -> Self {
        let s = unsafe { str::from_utf8_unchecked(value) };
        Self(s)
    }
}
