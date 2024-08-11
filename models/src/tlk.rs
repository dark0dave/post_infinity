use binrw::{io::Cursor, BinRead, BinReaderExt, BinWrite};
use serde::{Deserialize, Serialize};

use crate::{common::resref::Resref, model::Model};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Lookup {
    #[serde(flatten)]
    pub header: TLKHeader,
    #[br(count=header.count_of_entries)]
    pub entries: Vec<TLKEntry>,
    #[br(parse_with=binrw::helpers::until_eof, map = |s: Vec<u8>| read_tlk_strings(s, &entries))]
    #[bw(map = |x : &Vec<String>| x.iter().flat_map(|x: &String| x.as_bytes().to_vec()).collect::<Vec<u8>>())]
    pub tlk_strings: Vec<String>,
}

fn read_tlk_strings(s: Vec<u8>, entries: &Vec<TLKEntry>) -> Vec<String> {
    let mut out = Vec::with_capacity(entries.len());
    let mut start: usize = 0;
    for entry in entries {
        let end: usize = start + entry.length_of_this_string as usize;
        out.push(String::from_utf8_lossy(s.get(start..end).unwrap_or_default()).to_string());
        start = end;
    }
    out
}

impl Model for Lookup {
    fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        match reader.read_le() {
            Ok(res) => res,
            Err(err) => {
                panic!("Errored with {:?}, dumping buffer: {:?}", err, buffer);
            }
        }
    }

    fn name(&self, _lookup: &Lookup) -> String {
        todo!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut writer = Cursor::new(Vec::new());
        self.write_le(&mut writer).unwrap();
        writer.into_inner()
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm#tlkv1_Header
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct TLKHeader {
    #[br(count = 4)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.as_bytes())]
    pub signature: String,
    #[br(count = 4)]
    #[br(map = |s: Vec<u8>| String::from_utf8(s).unwrap_or_default())]
    #[bw(map = |x| x.as_bytes())]
    pub version: String,
    pub language_id: u16,
    pub count_of_entries: u32,
    pub offset_to_strings: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm#tlkv1_Entry
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
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
                signature: "TLK ".to_string(),
                version: "V1  ".to_string(),
                language_id: 0,
                count_of_entries: 34000,
                offset_to_strings: 884018,
            }
        );
        let entry = lookup.entries.get(400).expect("Failed to find entry");
        assert_eq!(
            entry,
            &TLKEntry {
                bit_field: 1,
                resource_name_of_associated_sound: Resref("\0\0\0\0\0\0\0\0".to_string()),
                volume_variance: 0,
                pitch_variance: 0,
                offset_to_this_string: 49264,
                length_of_this_string: 213,
            }
        );
        assert_eq!(lookup.tlk_strings.get(400), Some(&" 'Twas some three hundred years hence, but folk still cringe at the mention of the destruction at Ulcaster School. I've not met a soul who claims to know why it occurred, and none that were there are alive to say.".to_string()));

        let entry = lookup.entries.last().expect("Failed to find entry");
        assert_eq!(
            entry,
            &TLKEntry {
                bit_field: 1,
                resource_name_of_associated_sound: Resref("\0\0\0\0\0\0\0\0".to_string()),
                volume_variance: 0,
                pitch_variance: 0,
                offset_to_this_string: 3855179,
                length_of_this_string: 11,
            }
        );
        assert_eq!(lookup.tlk_strings.last(), Some(&"placeholder".to_string()))
    }
}
