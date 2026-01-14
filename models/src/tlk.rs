use core::{slice, str};
use std::error::Error;

use serde::{Deserialize, Serialize};
use zerocopy::FromBytes;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

const START_OF_ENTRIES: usize = 18_usize;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm
#[derive(Debug, PartialEq, Serialize, IntoBytes)]
#[repr(C, packed)]
pub struct TLK<'data> {
    #[serde(borrow)]
    pub header: &'data TLKHeader,
    #[serde(borrow)]
    pub entries: &'data [TLKEntry],
    #[serde(borrow)]
    pub strings: &'data [&'data str],
}

impl<'data> TLK<'data> {
    pub fn parse(bytes: &'data [u8]) -> Result<Self, Box<dyn Error>> {
        let (header, _) = <TLKHeader>::ref_from_prefix(bytes).map_err(|err| err.to_string())?;

        let source = bytes
            .get(START_OF_ENTRIES..)
            .ok_or("could not get entries from tlk buffer")?;
        let number_of_entries: usize = header.count_of_entries.try_into()?;
        let (entries, _) = <[TLKEntry]>::ref_from_prefix_with_elems(source, number_of_entries)
            .map_err(|err| err.to_string())?;
        let tlk_strings = bytes
            .get(header.offset_to_strings as usize..)
            .ok_or("could not get tlk strings from tlk buffer")?;

        let strings = unsafe {
            let layout = std::alloc::Layout::array::<&str>(header.count_of_entries as usize)?;
            let ptr = std::alloc::alloc(layout) as *mut &str;

            for (i, entry) in entries.iter().enumerate() {
                let start = entry.offset_to_this_string as usize;
                let end = start + entry.length_of_this_string as usize;
                if end <= bytes.len() {
                    let slice: &[u8] = tlk_strings
                        .get(start..end)
                        .ok_or("failed to get bytes for string from buffer")?;
                    *ptr.add(i) = std::str::from_utf8(slice)?;
                }
            }

            slice::from_raw_parts(ptr, header.count_of_entries as usize)
        };

        Ok(TLK {
            header,
            entries,
            strings,
        })
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm#tlkv1_Header
#[derive(
    Debug, PartialEq, Serialize, Deserialize, FromBytes, IntoBytes, Immutable, KnownLayout,
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
#[derive(
    Debug, PartialEq, Serialize, Deserialize, FromBytes, IntoBytes, Immutable, KnownLayout,
)]
#[repr(C, packed)]
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
        let tlk = TLK::parse(buffer.as_slice())?;
        assert_eq!(
            *tlk.header,
            TLKHeader {
                signature: "TLK ".as_bytes().try_into()?,
                version: "V1  ".as_bytes().try_into()?,
                language_id: 0,
                count_of_entries: 34000,
                offset_to_strings: 884018
            }
        );
        let entry = tlk.entries.get(400).expect("Failed to find entry");
        assert_eq!(
            entry,
            &TLKEntry {
                bit_field: 1,
                resource_name_of_associated_sound: [0; 8],
                volume_variance: 0,
                pitch_variance: 0,
                offset_to_this_string: 49264,
                length_of_this_string: 213,
            }
        );
        assert_eq!(
            tlk.strings.get(400),
            Some(
                &" 'Twas some three hundred years hence, but folk still cringe at the mention of the destruction at Ulcaster School. I've not met a soul who claims to know why it occurred, and none that were there are alive to say."
            )
        );

        let entry = tlk.entries.last().expect("Failed to find entry");
        assert_eq!(
            entry,
            &TLKEntry {
                bit_field: 1,
                resource_name_of_associated_sound: [0; 8],
                volume_variance: 0,
                pitch_variance: 0,
                offset_to_this_string: 3855179,
                length_of_this_string: 11,
            }
        );
        assert_eq!(tlk.strings.last(), Some(&"placeholder"));
        Ok(())
    }
}
