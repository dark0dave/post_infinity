use binrw::{BinRead, BinReaderExt, BinWrite, io::Cursor};
use serde::{Deserialize, Serialize};

use crate::common::Resref;
use crate::model::Model;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/eff_v1.htm#effv1_Header
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct EffectV1 {
    pub effect_type: u16,
    pub target_type: u8,
    pub power: u8,
    pub parameter_1: u32,
    pub parameter_2: u32,
    pub timing_mode: u8,
    pub dispel_resistance: u8,
    pub duration: u32,
    pub probability_1: u8,
    pub probability_2: u8,
    pub resref_key: Resref,
    pub dice_thrown_maximum_level: u32,
    pub dice_sides_minimum_level: u32,
    pub saving_throw_type: u32,
    pub saving_throw_bonus: u32,
    #[serde(skip)]
    _unknown: u32,
}

impl Model for EffectV1 {
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
