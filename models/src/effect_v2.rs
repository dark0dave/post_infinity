use binrw::{io::Cursor, BinRead, BinReaderExt, BinWrite};
use serde::{Deserialize, Serialize};

use crate::common::char_array::CharArray;
use crate::common::resref::Resref;
use crate::model::Model;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/eff_v2.htm
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct EffectV2 {
    #[br(count = 4)]
    pub signature: CharArray,
    #[br(count = 4)]
    pub version: CharArray,
    #[serde(flatten)]
    pub body: EffectV2Body,
}

impl Model for EffectV2 {
    fn new(buffer: &[u8]) -> Self {
        let tmp = if buffer.len() < 272 {
            let mut temp = buffer.to_vec();
            temp.extend([0_u8]);
            temp
        } else {
            buffer.to_vec()
        };
        let mut reader = Cursor::new(tmp);
        match reader.read_le() {
            Ok(res) => res,
            Err(err) => {
                panic!("Errored with {:?}, dumping buffer: {:?}", err, buffer);
            }
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut writer = Cursor::new(Vec::new());
        self.write_le(&mut writer).unwrap();
        writer.into_inner()
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/eff_v2.htm#effv2_Body
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct EffectV2Body {
    #[br(count = 4)]
    pub signature: CharArray,
    #[br(count = 4)]
    pub version: CharArray,
    #[serde(flatten)]
    pub body: EffectV2BodyWithOutHeader,
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/cre_v1.htm#CREV1_0_Effects
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct EffectV2BodyWithOutHeader {
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
    pub resource_1: Resref,
    pub dice_thrown: u32,
    pub dice_sides: u32,
    pub saving_throw_type: u32,
    pub saving_throw_bonus: u32,
    pub special: u32,
    pub primary_spell_school: u32,
    #[serde(skip)]
    _unknown_1: u32,
    pub parent_resource_lowest_affected_level: u32,
    pub parent_resource_highest_affected_level: u32,
    pub dispel_resistance: u32,
    pub parameter_3: u32,
    pub parameter_4: u32,
    pub parameter_5: u32,
    pub time_applied_ticks: u32,
    pub resource_2: Resref,
    pub resource_3: Resref,
    pub caster_x_coordinate: u32,
    pub caster_y_coordinate: u32,
    pub target_x_coordinate: u32,
    pub target_y_coordinate: u32,
    pub parent_resource_type: u32,
    pub parent_resource: Resref,
    #[br(count = 4)]
    pub parent_resource_flags: Vec<u8>,
    pub projectile: u32,
    pub parent_resource_slot: u32,
    #[br(count = 32)]
    pub variable_name: CharArray,
    pub caster_level: u32,
    pub first_apply: u32,
    // https://gibberlings3.github.io/iesdp/files/2da/2da_bgee/msectype.htm
    pub secondary_type: u32,
    #[serde(skip)]
    #[br(count = 15)]
    _unknown_2: Vec<u32>,
}

#[cfg(test)]
mod tests {

    use super::*;
    use binrw::io::{BufReader, Read};
    use pretty_assertions::assert_eq;
    use std::fs::File;

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
                signature: "EFF ".into(),
                version: "V2.0".into(),
                body: EffectV2Body {
                    signature: "EFF ".into(),
                    version: "V2.0".into(),
                    body: EffectV2BodyWithOutHeader {
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
                        resource_1: Resref("\0\0\0\0\0\0\0\0".into()),
                        dice_thrown: 0,
                        dice_sides: 0,
                        saving_throw_type: 0,
                        saving_throw_bonus: 0,
                        special: 0,
                        primary_spell_school: 0,
                        _unknown_1: 0,
                        parent_resource_lowest_affected_level: 0,
                        parent_resource_highest_affected_level: 0,
                        dispel_resistance: 0,
                        parameter_3: 5,
                        parameter_4: 0,
                        parameter_5: 0,
                        time_applied_ticks: 0,
                        resource_2: Resref("\0\0\0\0\0\0\0\0".into()),
                        resource_3: Resref("\0\0\0\0\0\0\0\0".into()),
                        caster_x_coordinate: 4294967295,
                        caster_y_coordinate: 4294967295,
                        target_x_coordinate: 4294967295,
                        target_y_coordinate: 4294967295,
                        parent_resource_type: 0,
                        parent_resource: Resref("\0\0\0\0\0\0\0\0".into()),
                        parent_resource_flags: vec![0; 4],
                        projectile: 0,
                        parent_resource_slot: 4294967295,
                        variable_name:
                            "\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"
                                .into(),
                        caster_level: 0,
                        first_apply: 0,
                        secondary_type: 0,
                        _unknown_2: vec![0; 15],
                    }
                },
            }
        )
    }
}
