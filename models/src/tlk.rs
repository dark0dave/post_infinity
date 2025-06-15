use core::str;

use serde::{Deserialize, Serialize};
use zerovec::{vecs::Index32, VarZeroVec, ZeroVec};

const START_OF_ENTRIES: usize = 18_usize;
const SIZE_OF_TLK_ENTRY: usize = 26_usize;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TLK<'data> {
    pub header: TLKHeader,
    #[serde(borrow)]
    pub entries: ZeroVec<'data, TLKEntry>,
    #[serde(borrow)]
    pub tlk_strings: VarZeroVec<'data, str, Index32>,
}

impl<'data> TLK<'data> {
    pub fn parse(bytes: &'data [u8]) -> Option<Self> {
        let header = TLKHeader::new(bytes);
        let end = START_OF_ENTRIES + (header.get_count_of_entries() as usize * SIZE_OF_TLK_ENTRY);
        let buff: &[u8] = bytes.get(START_OF_ENTRIES..end).unwrap_or_default();
        let entries: ZeroVec<'data, TLKEntry> = ZeroVec::parse_bytes(buff).unwrap();

        let mut buff = vec![];
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
        Some(TLK {
            header,
            entries,
            tlk_strings,
        })
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm#tlkv1_Header
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TLKHeader {
    pub signature: [u8; 4],
    pub version: [u8; 4],
    pub language_id: u16,
    pub count_of_entries: u32,
    pub offset_to_strings: u32,
}

// TODO: Convert to ZeroCopy
impl TLKHeader {
    fn new(bytes: &[u8]) -> Self {
        TLKHeader {
            signature: bytes
                .get(0..4)
                .unwrap_or_default()
                .try_into()
                .unwrap_or_default(),
            version: bytes
                .get(4..8)
                .unwrap_or_default()
                .try_into()
                .unwrap_or_default(),
            language_id: u16::from_le_bytes(
                bytes
                    .get(8..10)
                    .unwrap_or_default()
                    .try_into()
                    .unwrap_or_default(),
            ),
            count_of_entries: u32::from_le_bytes(
                bytes
                    .get(10..14)
                    .unwrap_or_default()
                    .try_into()
                    .unwrap_or_default(),
            ),
            offset_to_strings: u32::from_le_bytes(
                bytes
                    .get(14..18)
                    .unwrap_or_default()
                    .try_into()
                    .unwrap_or_default(),
            ),
        }
    }
    fn get_count_of_entries(&self) -> u32 {
        self.count_of_entries
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm#tlkv1_Entry
#[zerovec::make_ule(TLKEntryULE)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
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
    fn valid_tlk_header_parsed() {
        let mut file = File::open("fixtures/dialog.tlk").expect("Fixture missing");
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).unwrap();
        let tlk = TLK::parse(buffer.as_slice()).unwrap();
        assert_eq!(
            tlk.header,
            TLKHeader {
                signature: "TLK ".as_bytes().try_into().unwrap(),
                version: "V1  ".as_bytes().try_into().unwrap(),
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
                resource_name_of_associated_sound: [0; 8],
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
                resource_name_of_associated_sound: [0; 8],
                volume_variance: 0,
                pitch_variance: 0,
                offset_to_this_string: 3855179,
                length_of_this_string: 11,
            }
        );
        let end: usize = tlk.header.count_of_entries.try_into().unwrap();
        assert_eq!(tlk.tlk_strings.get(end - 1), Some("placeholder"))
    }
}
