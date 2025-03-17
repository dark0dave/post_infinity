use binrw::{io::Cursor, BinRead, BinReaderExt, BinWrite};
use serde::{Deserialize, Serialize};

use crate::common::char_array::CharArray;
use crate::common::header::Header;
use crate::common::Resref;
use crate::model::Model;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/eff_v2.htm
#[derive(Debug, PartialEq, BinRead, BinWrite, Serialize, Deserialize)]
pub struct EffectV2 {
    #[serde(flatten)]
    pub header: Header,
    pub effect: EffectV2Body,
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
    #[serde(flatten)]
    pub header: Header,
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
    pub variable_name: CharArray<32>,
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
    use binrw::io::Read;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use std::{error::Error, fs::File};

    const FIXTURES: [(&str, &str); 1] = [("fixtures/#trollis.eff", "fixtures/#trollis.eff.json")];

    fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    #[test]
    fn parse() -> Result<(), Box<dyn Error>> {
        for (file_path, json_file_path) in FIXTURES {
            let effect: EffectV2 = EffectV2::new(&read_file(file_path)?);
            let result: Value = serde_json::to_value(effect)?;
            let expected: Value = serde_json::from_slice(&read_file(json_file_path)?)?;

            assert_eq!(result, expected);
        }
        Ok(())
    }
}
