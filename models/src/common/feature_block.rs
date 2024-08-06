use binrw::{BinRead, BinWrite};
use serde::{Deserialize, Serialize};

use super::resref::Resref;

#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct FeatureBlock {
    pub opcode_number: u16,
    pub target_type: u8,
    pub power: u8,
    pub parameter_1: u32,
    pub parameter_2: u32,
    pub timing_mode: u8,
    pub dispel_resistance: u8,
    pub duration: u32,
    pub probability_1: u8,
    pub probability_2: u8,
    pub resource: Resref,
    pub dice_thrown_max_level: u32,
    pub dice_sides_min_level: u32,
    #[br(count = 4)]
    pub saving_throw_type: Vec<u8>,
    pub saving_throw_bonus: u32,
    pub stacking_id: u32,
}
