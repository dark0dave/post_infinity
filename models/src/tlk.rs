use std::fs::File;

use binrw::{
    io::{BufReader, Read, Seek},
    BinRead, BinReaderExt, BinResult, BinWrite,
};
use serde::{Deserialize, Serialize};

use crate::common::{char_array::CharArray, resref::Resref};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm
#[derive(Debug, BinRead, BinWrite, Serialize, Deserialize)]
pub struct TLK {
    #[serde(flatten)]
    pub header: TLKHeader,
    #[br(count=header.count_of_entries)]
    pub entries: Vec<TLKEntry>,
    #[br(parse_with = |reader, _, _:()| read_to_end(reader, &entries))]
    #[bw(map = |x : &Vec<CharArray>| x.iter().flat_map(|x: &CharArray| x.0.clone()).collect::<Vec<u8>>())]
    pub tlk_strings: Vec<CharArray>,
}

fn read_to_end<R: Read + Seek>(
    reader: &mut R,
    entries: &Vec<TLKEntry>,
) -> BinResult<Vec<CharArray>> {
    let mut out = vec![];
    let mut buf = vec![];
    reader.read_to_end(&mut buf)?;
    for entry in entries {
        let start = entry.offset_to_this_string as usize;
        let end = start + entry.length_of_this_string as usize;
        out.push(CharArray(buf.get(start..end).unwrap_or_default().to_vec()));
    }
    Ok(out)
}

impl TLK {
    pub fn new(reader: &mut BufReader<File>) -> Self {
        match reader.read_le() {
            Ok(res) => res,
            Err(err) => {
                panic!("Errored with {:?}", err);
            }
        }
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/tlk_v1.htm#tlkv1_Header
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct TLKHeader {
    #[br(count = 4)]
    pub signature: CharArray,
    #[br(count = 4)]
    pub version: CharArray,
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
    use pretty_assertions::assert_eq;

    #[test]
    fn valid_lookup_parsed() {
        let file = File::open("fixtures/dialog.tlk").expect("Fixture missing");
        let mut reader = BufReader::new(file);
        let tlk = TLK::new(&mut reader);
        assert_eq!(
            tlk.header,
            TLKHeader {
                signature: "TLK ".into(),
                version: "V1  ".into(),
                language_id: 0,
                count_of_entries: 34000,
                offset_to_strings: 884018,
            }
        );
        let entry = tlk.entries.get(400).expect("Failed to find entry");
        assert_eq!(
            entry,
            &TLKEntry {
                bit_field: 1,
                resource_name_of_associated_sound: Resref("\0\0\0\0\0\0\0\0".into()),
                volume_variance: 0,
                pitch_variance: 0,
                offset_to_this_string: 49264,
                length_of_this_string: 213,
            }
        );
        assert_eq!(tlk.tlk_strings.get(400), Some(&" 'Twas some three hundred years hence, but folk still cringe at the mention of the destruction at Ulcaster School. I've not met a soul who claims to know why it occurred, and none that were there are alive to say.".into()));

        let entry = tlk.entries.last().expect("Failed to find entry");
        assert_eq!(
            entry,
            &TLKEntry {
                bit_field: 1,
                resource_name_of_associated_sound: Resref("\0\0\0\0\0\0\0\0".into()),
                volume_variance: 0,
                pitch_variance: 0,
                offset_to_this_string: 3855179,
                length_of_this_string: 11,
            }
        );
        assert_eq!(tlk.tlk_strings.last(), Some(&"placeholder".into()))
    }
}
