use serde::Serialize;

use super::fixed_char_array::FixedCharSlice;

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, Serialize)]
pub struct FeatureBlock {
    pub opcode_number: u16,
    pub target_type: u8,
    pub power: u8,
    pub parameter_1: u32,
    pub parameter_2: u32,
    pub timing_mode: u8,
    pub dispel_resistance: u8,
    pub duration: u32,
    pub propability_1: u8,
    pub propability_2: u8,
    pub resource: FixedCharSlice<8>,
    pub dice_thrown_max_level: u32,
    pub dice_sides_min_level: u32,
    pub saving_throw_type: FixedCharSlice<4>,
    pub saving_throw_bonus: u32,
    pub stacking_id: u32,
}
