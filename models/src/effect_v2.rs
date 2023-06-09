use std::{mem::size_of, rc::Rc};

use serde::Serialize;

use crate::common::header::Header;
use crate::resources::utils::copy_buff_to_struct;
use crate::tlk::Lookup;
use crate::{common::fixed_char_array::FixedCharSlice, model::Model};

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct EffectV2 {
    // https://gibberlings3.github.io/iesdp/file_formats/ie_formats/eff_v2.htm
    pub effect_v2_header: Header<4, 4>,
    pub body: EffectV2Body,
}

impl Model for EffectV2 {
    fn new(buffer: &[u8]) -> Self {
        let effect_v2_header = copy_buff_to_struct::<Header<4, 4>>(buffer, 0);
        let body = copy_buff_to_struct::<EffectV2Body>(buffer, size_of::<Header<4, 4>>());
        Self {
            effect_v2_header,
            body,
        }
    }

    fn create_as_rc(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }

    fn name(&self, _lookup: &Lookup) -> String {
        todo!()
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/eff_v2.htm#effv2_Body
#[repr(C, packed)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize)]
pub struct EffectV2Body {
    pub header: Header<4, 4>,
    pub opcode_number: u32,
    pub target_type: u32,
    pub power: u32,
    pub parameter_1: u32,
    pub parameter_2: u32,
    pub timing_mode: u16,
    pub timing: u16,
    pub duration: u32,
    pub probability_1: u16,
    pub probability_2: u16,
    pub resource_1: FixedCharSlice<8>,
    pub dice_thrown: u32,
    pub dice_sides: u32,
    pub saving_throw_type: u32,
    pub saving_throw_bonus: u32,
    pub speacial: u32,
    pub primary_spell_school: u32,
    #[serde(skip_serializing)]
    _unknown_1: u32,
    pub parent_resource_lowest_affected_level: u32,
    pub parent_resource_highest_affected_level: u32,
    pub dispel_resistance: u32,
    pub parameter_3: u32,
    pub parameter_4: u32,
    pub parameter_5: u32,
    pub time_applied_ticks: u32,
    pub resource_2: FixedCharSlice<8>,
    pub resource_3: FixedCharSlice<8>,
    pub caster_x_coordinate: i32,
    pub caster_y_coordinate: i32,
    pub target_x_coordinate: i32,
    pub target_y_coordinate: i32,
    pub parent_resource_type: u32,
    pub parent_resource: FixedCharSlice<8>,
    pub parent_resource_flags: FixedCharSlice<4>,
    pub projectile: u32,
    pub parent_resource_slot: i32,
    pub variable_name: FixedCharSlice<32>,
    pub caster_level: u32,
    pub first_apply: u32,
    // https://gibberlings3.github.io/iesdp/files/2da/2da_bgee/msectype.htm
    pub secondary_type: u32,
    // Drop last u8 breaks BGEE parsing
    #[serde(skip_serializing)]
    _unknown_2: [u32; 14],
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    #[test]
    fn valid_simple_creature_file_header_parsed() {
        let file = File::open("fixtures/#trollis.eff").unwrap();

        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();

        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");
        assert_eq!(
            EffectV2::new(&buffer),
            EffectV2 {
                effect_v2_header: Header {
                    signature: "EFF ".into(),
                    version: "V2.0".into(),
                },
                body: EffectV2Body {
                    header: Header {
                        signature: "EFF ".into(),
                        version: "V2.0".into(),
                    },
                    opcode_number: 98,
                    target_type: 2,
                    power: 0,
                    parameter_1: 6,
                    parameter_2: 4,
                    timing_mode: 0,
                    timing: 0,
                    duration: 120,
                    probability_1: 100,
                    probability_2: 0,
                    resource_1: FixedCharSlice::default(),
                    dice_thrown: 0,
                    dice_sides: 0,
                    saving_throw_type: 0,
                    saving_throw_bonus: 0,
                    speacial: 0,
                    primary_spell_school: 0,
                    _unknown_1: 0,
                    parent_resource_lowest_affected_level: 0,
                    parent_resource_highest_affected_level: 0,
                    dispel_resistance: 0,
                    parameter_3: 5,
                    parameter_4: 0,
                    parameter_5: 0,
                    time_applied_ticks: 0,
                    resource_2: FixedCharSlice::default(),
                    resource_3: FixedCharSlice::default(),
                    caster_x_coordinate: -1,
                    caster_y_coordinate: -1,
                    target_x_coordinate: -1,
                    target_y_coordinate: -1,
                    parent_resource_type: 0,
                    parent_resource: FixedCharSlice::default(),
                    parent_resource_flags: FixedCharSlice::default(),
                    projectile: 0,
                    parent_resource_slot: -1,
                    variable_name: "".into(),
                    caster_level: 0,
                    first_apply: 0,
                    secondary_type: 0,
                    _unknown_2: [0; 14],
                },
            }
        )
    }
}
