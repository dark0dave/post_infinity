use binrw::{io::Cursor, io::Read, BinRead, BinReaderExt, BinWrite};
use flate2::bufread::ZlibDecoder;
use serde::{Deserialize, Serialize};

use crate::{common::char_array::CharArray, model::Model};

// "BAM\0"
const BAM_SIGNATURE: &[u8; 4] = &[66, 65, 77, 0];
// "BAMC"
const BAMC_SIGNATURE: &[u8; 4] = &[66, 65, 77, 67];
// "v1  "
const VERSION1: &[u8; 4] = &[118, 49, 32, 32];
// "v2  "
const VERSION2: &[u8; 4] = &[118, 50, 32, 32];

// This is slow
// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bam_v1.htm
// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bam_v2.htm
// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bamcv1.htm
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct Bam {
    #[serde(flatten)]
    pub header: BamHeader,
    // If BAM v1
    #[serde(flatten)]
    #[brw(if(header.signature.0 == BAM_SIGNATURE && header.version.0 == VERSION1))]
    pub bamv1header: BamV1Header,
    #[brw(if(header.signature.0 == BAM_SIGNATURE && header.version.0 == VERSION1))]
    #[br(count=bamv1header.count_of_frame_entries)]
    pub bamv1_frame_entries: Vec<BamV1FrameEntry>,
    #[brw(if(header.signature.0 == BAM_SIGNATURE && header.version.0 == VERSION1))]
    #[br(count=bamv1header.count_of_cycles)]
    pub bamv1_cycle_entries: Vec<CycleEntry>,
    // https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bam_v1.htm#bamv1_Palette
    #[brw(if(header.signature.0 == BAM_SIGNATURE && header.version.0 == VERSION1))]
    #[br(count=bamv1header.offset_to_lookup_table-bamv1header.offset_to_palette)]
    pub bamv1_palette: Vec<u8>,
    // https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bam_v1.htm#bamv1_FrameLUT
    #[brw(if(header.signature.0 == BAM_SIGNATURE && header.version.0 == VERSION1))]
    #[br(count=lookup_table_size(&bamv1_cycle_entries))]
    pub bamv1_lookup_table: Vec<u8>,
    // https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bam_v1.htm#bamv1_Data
    #[brw(if(header.signature.0 == BAM_SIGNATURE && header.version.0 == VERSION1))]
    #[br(parse_with=binrw::helpers::until_eof)]
    pub bamv1_frame_data: Vec<u8>,
    // If BAMC
    #[brw(if(header.signature.0 == BAMC_SIGNATURE))]
    pub uncompressed_length: u32,
    #[brw(if(header.signature.0 == BAMC_SIGNATURE))]
    #[br(parse_with=binrw::helpers::until_eof, restore_position)]
    pub compressed_data: Vec<u8>,
    #[brw(if(header.signature.0 == BAMC_SIGNATURE))]
    #[br(parse_with=binrw::helpers::until_eof, map = |s: Vec<u8>| parse_compressed_data(&s))]
    #[bw(ignore)]
    pub uncompressed_data: Vec<u8>,
    // If BAM v2
    #[serde(flatten)]
    #[brw(if(header.signature.0 == BAM_SIGNATURE && header.version.0 == VERSION2))]
    pub bamv2header: BamV2Header,
    #[brw(if(header.signature.0 == BAM_SIGNATURE && header.version.0 == VERSION2))]
    #[br(count=bamv2header.count_of_frame_entries)]
    pub bamv2_frame_entries: Vec<BamV2FrameEntry>,
    // https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bam_v2.htm#bamv2_CycleEntry
    #[brw(if(header.signature.0 == BAM_SIGNATURE && header.version.0 == VERSION2))]
    #[br(count=bamv2header.count_of_cycle_entries)]
    pub bamv2_cycle_entries: Vec<CycleEntry>,
    #[brw(if(header.signature.0 == BAM_SIGNATURE && header.version.0 == VERSION2))]
    #[br(count=bamv2header.count_of_data_blocks)]
    pub bamv2_data_blocks: Vec<DataBlock>,
}

// To find the number of entries in this lookup table,
// find the largest value of start+count in the cycle entries table.
fn lookup_table_size(cycle_entries: &[CycleEntry]) -> u64 {
    cycle_entries.iter().fold(0, |accum, entry| {
        let size_entry = entry.index_into_frame_lookup_table + entry.count_of_frame_indices;
        if accum > size_entry {
            accum
        } else {
            size_entry
        }
    }) as u64
}

fn parse_compressed_data(buff: &[u8]) -> Vec<u8> {
    let mut d = ZlibDecoder::new(buff);
    let mut buffer = vec![];
    if let Err(err) = d.read_to_end(&mut buffer) {
        println!("{}", err);
    }
    buffer
}

impl Model for Bam {
    fn new(buffer: &[u8]) -> Self {
        let mut reader = Cursor::new(buffer);
        reader.read_le().unwrap()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut writer = Cursor::new(Vec::new());
        self.write_le(&mut writer).unwrap();
        writer.into_inner()
    }
}

// Generic header for this one
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct BamHeader {
    #[br(count = 4)]
    pub signature: CharArray,
    #[br(count = 4)]
    pub version: CharArray,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bam_v1.htm#bamv1_Header
#[derive(Debug, Default, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct BamV1Header {
    pub count_of_frame_entries: u16,
    // Yes its unsized for some horrible reason
    pub count_of_cycles: i8,
    // Yes its unsized for some horrible reason
    pub compressed_color_index: i8,
    // Offset (from start of file) to frame entries (which are immediately followed by cycle entries)
    pub offset_to_frame_entries: u32,
    // Offset (from start of file) to palette
    pub offset_to_palette: u32,
    // Offset (from start of file) to frame lookup table
    pub offset_to_lookup_table: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bam_v1.htm#bamv1_FrameEntry
#[derive(Debug, Default, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct BamV1FrameEntry {
    pub frame_width: u16,
    pub frame_hieght: u16,
    // Yes its unsized for some horrible reason
    pub frame_center_x_coordinate: i16,
    // Yes its unsized for some horrible reason
    pub frame_center_y_coordinate: i16,
    // bits 30-0: Offset to frame data, bit 31: 0=Compressed (RLE), 1=Uncompressed
    pub offset_to_frame_data: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bam_v1.htm#bamv1_CycleEntry
#[derive(Debug, Default, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct CycleEntry {
    pub count_of_frame_indices: u16,
    pub index_into_frame_lookup_table: u16,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bam_v2.htm#bamv2_Header
#[derive(Debug, Default, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct BamV2Header {
    pub count_of_frame_entries: u32,
    pub count_of_cycle_entries: u32,
    pub count_of_data_blocks: u32,
    // Offset (from start of file) to frame entries
    pub offset_to_frame_entries: u32,
    // Offset (from start of file) to cycle entries
    pub offset_to_cycle_entries: u32,
    // Offset (from start of file) to data blocks
    pub offset_to_data_blocks: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bam_v2.htm#bamv2_FrameEntry
#[derive(Debug, Default, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct BamV2FrameEntry {
    pub frame_width: u16,
    pub frame_hieght: u16,
    // Yes its unsized for some horrible reason
    pub frame_center_x_coordinate: i8,
    // Yes its unsized for some horrible reason
    pub frame_center_y_coordinate: i8,

    pub compressed_color_index: i8,
    // bits 30-0: Offset to frame data, bit 31: 0=Compressed (RLE), 1=Uncompressed
    pub offset_to_frame_data: u32,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/bam_v2.htm#bamv2_DataBlock
#[derive(Debug, Default, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct DataBlock {
    pub pvrz_page: u32,
    pub source_x_coordinate: u32,
    pub source_y_coordinate: u32,
    pub width: u32,
    pub height: u32,
    pub target_x_coordinate: u32,
    pub target_y_coordinate: u32,
}
