use std::rc::Rc;

use serde::Serialize;

use crate::common::fixed_char_array::FixedCharSlice;
use crate::model::Model;
use crate::resources::utils::copy_buff_to_struct;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/eff_v1.htm#effv1_Header
#[derive(Debug, Serialize)]
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
    pub resref_key: FixedCharSlice<8>,
    pub dice_thrown_maximum_level: u32,
    pub dice_sides_minimmum_level: u32,
    pub saving_throw_type: u32,
    pub saving_throw_bonus: u32,
    #[serde(skip_serializing)]
    _unknown: u32,
}

impl Model for EffectV1 {
    fn new(buffer: &[u8]) -> Self {
        copy_buff_to_struct::<EffectV1>(buffer, 0)
    }

    fn create_as_box(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }
}
