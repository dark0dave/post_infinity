use serde::Serialize;

use crate::{
    common::{
        header::Header, signed_fixed_char_array::SignedFixedCharSlice,
        varriable_char_array::VarriableCharArray,
    },
    resources::utils::{copy_buff_to_struct, copy_transmute_buff},
};

#[derive(Debug, Serialize)]
pub struct Lookup {
    pub header: TLKHeader,
    pub data_entries: Vec<TLKDataEntry>,
}

impl Lookup {
    pub fn new(buffer: &[u8]) -> Self {
        let header = copy_buff_to_struct::<TLKHeader>(buffer, 0);

        // This is hard coded by the file format
        let start = 18;
        let count = usize::try_from(header.count_of_entries).unwrap_or(0);
        let entries = copy_transmute_buff::<TLKEntry>(buffer, start, count);

        let start = usize::try_from(header.offset_to_strings).unwrap_or(0);
        let data_entries = entries
            .iter()
            .map(|entry| TLKDataEntry::new(start, entry, buffer))
            .collect();

        Self {
            header,
            data_entries,
        }
    }
}

//https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm#tlkv1_Header
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
pub struct TLKHeader {
    pub header: Header<4, 4>,
    pub language_id: i16,
    pub count_of_entries: u32,
    pub offset_to_strings: u32,
}

//https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm#tlkv1_Entry
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
pub struct TLKEntry {
    /*
        00 - No message data
        01 - Text exists
        02 - Sound exists
        03 - Standard message. Ambient message. Used for sound without text (BG1) or message displayed over characters head (BG2) , Message with tags (for instance <CHARNAME>) for all games except BG2
        04 - Token exists (for instance <CHARNAME>), BG2 and EEs only
    */
    pub bit_field: i16,
    pub resource_name_of_associated_sound: SignedFixedCharSlice<8>,
    //  Unused, at minimum in BG1
    pub volume_variance: u32,
    // Unused, at minimum in BG1
    pub pitch_variance: u32,
    pub offset_of_this_string_relative_to_the_strings_section: u32,
    pub length_of_this_string: u32,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TLKDataEntry {
    pub entry: TLKEntry,
    pub strings: VarriableCharArray,
}

impl TLKDataEntry {
    fn new(start: usize, entry: &TLKEntry, buffer: &[u8]) -> Self {
        let buff_start = start
            + usize::try_from(entry.offset_of_this_string_relative_to_the_strings_section)
                .unwrap_or(0);
        let buff_end = buff_start + usize::try_from(entry.length_of_this_string).unwrap_or(0);
        let strings = VarriableCharArray(buffer.get(buff_start..buff_end).unwrap().to_vec());

        TLKDataEntry {
            entry: *entry,
            strings,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    #[test]
    fn valid_lookup_parsed() {
        let file = File::open("fixtures/dialog.tlk").expect("Fixture missing");
        let mut buffer = Vec::new();
        BufReader::new(file)
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");

        let lookup = Lookup::new(&buffer);
        assert_eq!(
            lookup.header,
            TLKHeader {
                header: Header {
                    signature: "TLK ".into(),
                    version: "V1  ".into(),
                },
                language_id: 0,
                count_of_entries: 34000,
                offset_to_strings: 884018,
            }
        );
        let entry = lookup.data_entries.get(400).expect("Failed to find entry");
        assert_eq!(
            entry,
            &TLKDataEntry {
                entry: TLKEntry {
                    bit_field: 1,
                    resource_name_of_associated_sound: "".into(),
                    volume_variance: 0,
                    pitch_variance: 0,
                    offset_of_this_string_relative_to_the_strings_section: 49264,
                    length_of_this_string: 213
                },
                strings: " 'Twas some three hundred years hence, but folk still cringe at the mention of the destruction at Ulcaster School. I've not met a soul who claims to know why it occurred, and none that were there are alive to say.".into()
            }
        )
    }
}
