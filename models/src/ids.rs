use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::{
    common::{
        fixed_char_array::FixedCharSlice, header::Header, variable_char_array::VariableCharArray,
    },
    model::Model,
    resources::utils::{row_parser, to_u8_slice, vec_to_u8_slice},
    tlk::Lookup,
};

//https://gibberlings3.github.io/iesdp/file_formats/ie_formats/ids.htm
#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Ids {
    pub header: Header<3, 4>,
    pub data_entries: Vec<DataEntry>,
}

impl Model for Ids {
    fn new(buffer: &[u8]) -> Self {
        let (headers, mut end) = row_parser(buffer, 0);

        let signature = if headers.first().is_some() {
            FixedCharSlice::<3>::from(&buffer[0..3])
        } else {
            FixedCharSlice::<3>::default()
        };
        let version = match headers.last() {
            Some(x) => FixedCharSlice::<4>::from(x.0.as_ref()),
            _ => FixedCharSlice::<4>::from(signature.0.as_ref()),
        };
        let header = Header { signature, version };

        let mut data_entries = vec![];
        while end < buffer.len() {
            let (row, row_end) = row_parser(buffer, end);
            if !row.is_empty() {
                data_entries.push(DataEntry {
                    value: row.first().unwrap().clone(),
                    identifier: row.last().unwrap().clone(),
                });
            }
            if end == row_end {
                break;
            }
            end = row_end;
        }
        Ids {
            header,
            data_entries,
        }
    }

    fn create_as_rc(buffer: &[u8]) -> Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }

    fn name(&self, _lookup: &Lookup) -> String {
        todo!()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut out = to_u8_slice(&self.header).to_vec();
        out.extend(vec_to_u8_slice(&self.data_entries));
        out
    }
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataEntry {
    pub value: VariableCharArray,
    pub identifier: VariableCharArray,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    #[test]
    fn valid_item_file_parsed() {
        let file = File::open("fixtures/soundoff.ids").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");
        let item = Ids::new(&buffer);

        assert_eq!(
            item.header,
            Header {
                version: "V1.0".into(),
                signature: "IDS".into(),
            }
        );

        assert_eq!(
            item.data_entries.first(),
            Some(&DataEntry {
                value: "0".into(),
                identifier: "INITIAL_MEETING".into(),
            })
        );
        assert_eq!(
            item.data_entries.last(),
            Some(&DataEntry {
                value: "13".into(),
                identifier: "BATTLE_CRY5".into(),
            })
        );
    }
}
