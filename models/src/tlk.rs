use core::str;
use std::{
    borrow::Cow::{self, Borrowed},
    error::Error,
};

use serde::{Deserialize, Serialize};
use zerovec::{vecs::Index32, VarZeroVec, ZeroVec};

use crate::common::{header::Header, Resref};

const START_OF_ENTRIES: usize = 18_usize;
const SIZE_OF_TLK_ENTRY: usize = 26_usize;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TLK<'data> {
    #[serde(flatten, borrow)]
    pub header: Cow<'data, TLKHeader>,
    #[serde(borrow)]
    pub entries: ZeroVec<'data, TLKEntry>,
    #[serde(borrow)]
    pub tlk_strings: VarZeroVec<'data, str, Index32>,
}

impl<'data> TLK<'data> {
    pub fn parse(bytes: &'data [u8]) -> Result<Self, Box<dyn Error>> {
        let header: &TLKHeader = unsafe {
            std::ptr::from_ref(bytes)
                .cast::<TLKHeader>()
                .as_ref()
                .ok_or("Failed to parse TLKHeader")?
        };
        let end = START_OF_ENTRIES + (header.count_of_entries as usize * SIZE_OF_TLK_ENTRY);
        let buff: &[u8] = bytes.get(START_OF_ENTRIES..end).unwrap_or_default();
        let entries: ZeroVec<'data, TLKEntry> = ZeroVec::parse_bytes(buff)?;
        let mut buff= vec![];
        // List through all the entries to find the strings
        for entry in entries.iter() {
            let start = (header.offset_to_strings + entry.offset_to_this_string) as usize;
            let end = start + entry.length_of_this_string as usize;
            if end <= bytes.len() {
                let slice: &[u8] = bytes.get(start..end).unwrap_or_default();
                let s = std::str::from_utf8(slice).unwrap_or_default();
                buff.push(s);
            }
        }
        let tlk_strings = VarZeroVec::from(&buff);

        Ok(TLK {
            header: Borrowed(header),
            entries,
            tlk_strings,
        })
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let out = vec![];
        return out;
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm#tlkv1_Header
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct TLKHeader {
    #[serde(flatten)]
    pub header: Header,
    pub language_id: u16,
    pub count_of_entries: u32,
    pub offset_to_strings: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm#tlkv1_Entry
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
#[zerovec::make_ule(TLKEntryULE)]
pub struct TLKEntry {
    /*
        00 - No message data
        01 - Text exists
        02 - Sound exists
        03 - Standard message. Ambient message. Used for sound without text (BG1) or message displayed over characters head (BG2) , Message with tags (for instance <CHARNAME>) for all games except BG2
        04 - Token exists (for instance <CHARNAME>), BG2 and EEs only
    */
    pub bit_field: u16,
    pub resource_name_of_associated_sound: Resref,
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
        let tlk = TLK::parse(buffer.as_slice())?;
        let header = tlk.header.into_owned();
        assert_eq!(
            header,
            TLKHeader {
                header: Header {
                    signature: "TLK ".into(),
                    version: "V1  ".into(),
                },
                language_id: 0,
                count_of_entries: 34000,
                offset_to_strings: 884018
            }
        );
        let entry = tlk.entries.get(400).expect("Failed to find entry");
        assert_eq!(
            entry,
            TLKEntry {
                bit_field: 1,
                resource_name_of_associated_sound: Resref::default(),
                volume_variance: 0,
                pitch_variance: 0,
                offset_to_this_string: 49264,
                length_of_this_string: 213,
            }
        );
        assert_eq!(tlk.tlk_strings.get(400), Some(" 'Twas some three hundred years hence, but folk still cringe at the mention of the destruction at Ulcaster School. I've not met a soul who claims to know why it occurred, and none that were there are alive to say."));

        let entry = tlk.entries.last().expect("Failed to find entry");
        assert_eq!(
            entry,
            TLKEntry {
                bit_field: 1,
                resource_name_of_associated_sound: Resref::default(),
                volume_variance: 0,
                pitch_variance: 0,
                offset_to_this_string: 3855179,
                length_of_this_string: 11,
            }
        );
        let end: usize = header.count_of_entries.try_into()?;
        assert_eq!(tlk.tlk_strings.get(end - 1), Some("placeholder"));
        Ok(())
    }
}
