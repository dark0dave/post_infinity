use core::str;
use std::error::Error;

use serde::{Deserialize, Serialize};
use zerocopy::{FromBytes, IntoBytes};
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};
use zerovec::{VarZeroVec, ZeroVec, make_ule, vecs::Index32};

use crate::model::Parseable;

const START_OF_ENTRIES: usize = 18_usize;
const SIZE_OF_TLK_ENTRY: usize = 26_usize;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TLK<'data> {
    #[serde(flatten)]
    pub header: TLKHeader,
    #[serde(borrow)]
    pub entries: ZeroVec<'data, TLKEntry>,
    #[serde(borrow)]
    pub strings: VarZeroVec<'data, str, Index32>,
}
impl<'data> Parseable<'data> for TLK<'data> {}

impl<'data> TryFrom<&'data [u8]> for TLK<'data> {
    type Error = Box<dyn Error>;

    fn try_from(bytes: &'data [u8]) -> Result<Self, Box<dyn Error>> {
        let (header, _) = <TLKHeader>::read_from_prefix(bytes).map_err(|err| err.to_string())?;

        let count_of_entries: usize = header.count_of_entries.try_into()?;
        let end = START_OF_ENTRIES + (count_of_entries * SIZE_OF_TLK_ENTRY);
        let buff: &[u8] = bytes
            .get(START_OF_ENTRIES..end)
            .ok_or("could not get entries from tlk buffer")?;
        let entries: ZeroVec<'data, TLKEntry> = ZeroVec::parse_bytes(buff)?;

        let mut buff = Vec::with_capacity(count_of_entries);
        // List through all the entries to find the strings
        for entry in entries.iter() {
            let start = (header.offset_to_strings + entry.offset_to_this_string).try_into()?;
            let end = start + entry.length_of_this_string as usize;
            if end <= bytes.len() {
                let slice: &[u8] = bytes.get(start..end).unwrap_or_default();
                let s = std::str::from_utf8(slice).unwrap_or_default();
                buff.push(s);
            }
        }
        let strings = VarZeroVec::from(&buff);

        Ok(TLK {
            header,
            entries,
            strings,
        })
    }
}

impl<'data> TryInto<Vec<u8>> for TLK<'data> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut buffer = vec![];
        self.header
            .write_to(&mut buffer)
            .map_err(|err| err.to_string())?;
        buffer.extend_from_slice(self.entries.as_bytes());
        buffer.extend_from_slice(self.strings.as_bytes());
        Ok(buffer)
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm#tlkv1_Header
#[derive(
    Debug, PartialEq, Serialize, Deserialize, FromBytes, IntoBytes, Immutable, KnownLayout, Clone,
)]
#[repr(C, packed)]
pub struct TLKHeader {
    pub signature: [u8; 4],
    pub version: [u8; 4],
    pub language_id: u16,
    pub count_of_entries: u32,
    pub offset_to_strings: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm#tlkv1_Entry
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
#[make_ule(TLKEntryULE)]
pub struct TLKEntry {
    /*
        00 - No message data
        01 - Text exists
        02 - Sound exists
        03 - Standard message. Ambient message. Used for sound without text (BG1) or message displayed over characters head (BG2) , Message with tags (for instance <CHARNAME>) for all games except BG2
        04 - Token exists (for instance <CHARNAME>), BG2 and EEs only
    */
    pub bit_field: u16,
    pub resource_name_of_associated_sound: [u8; 8],
    //  Unused, at minimum in BG1
    pub volume_variance: u32,
    // Unused, at minimum in BG1
    pub pitch_variance: u32,
    // Offset of this string relative to the strings section
    pub offset_to_this_string: u32,
    pub length_of_this_string: u32,
}

#[cfg(test)]
mod tests {

    use std::{fs::File, io::Read};

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn valid_tlk_header_parsed() -> Result<(), Box<dyn Error>> {
        let mut file = File::open("fixtures/dialog.tlk").expect("Fixture missing");
        let mut buffer = vec![];
        file.read_to_end(&mut buffer)?;
        let tlk = TLK::try_from(buffer.as_slice())?;
        let entries = { tlk.entries };
        let strings = { tlk.strings };
        assert_eq!(
            tlk.header,
            TLKHeader {
                signature: "TLK ".as_bytes().try_into()?,
                version: "V1  ".as_bytes().try_into()?,
                language_id: 0,
                count_of_entries: 34000,
                offset_to_strings: 884018
            }
        );
        let entry = entries.get(400).expect("Failed to find entry");
        assert_eq!(
            entry,
            TLKEntry {
                bit_field: 1,
                resource_name_of_associated_sound: [0; 8],
                volume_variance: 0,
                pitch_variance: 0,
                offset_to_this_string: 49264,
                length_of_this_string: 213,
            }
        );
        assert_eq!(
            strings.get(400),
            Some(
                " 'Twas some three hundred years hence, but folk still cringe at the mention of the destruction at Ulcaster School. I've not met a soul who claims to know why it occurred, and none that were there are alive to say."
            )
        );

        let entry = entries.last().expect("Failed to find entry");
        assert_eq!(
            entry,
            TLKEntry {
                bit_field: 1,
                resource_name_of_associated_sound: [0; 8],
                volume_variance: 0,
                pitch_variance: 0,
                offset_to_this_string: 3855179,
                length_of_this_string: 11,
            }
        );
        assert_eq!(strings.get(strings.len() - 1), Some("placeholder"));
        Ok(())
    }
}
