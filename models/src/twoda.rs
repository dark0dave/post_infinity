use std::{rc::Rc, vec};

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::common::fixed_char_array::FixedCharSlice;
use crate::common::header::Header;
use crate::common::variable_char_array::VariableCharArray;
use crate::model::Model;
use crate::resources::utils::{row_parser, to_u8_slice, vec_to_u8_slice};
use crate::tlk::Lookup;

//https://gibberlings3.github.io/iesdp/file_formats/ie_formats/2da.htm
#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoDA {
    pub header: Header<3, 4>,
    pub default_value: VariableCharArray,
    pub data_entries: DataEntry,
}

impl Model for TwoDA {
    fn new(buffer: &[u8]) -> Self {
        // Parse Headers
        let (headers, end) = row_parser(buffer, 0);

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

        // Parse Default Value
        let (default_values, end) = row_parser(buffer, end);

        // Parse Data Entry Headers
        let (data_entry_headers, mut end) = row_parser(buffer, end);

        // Parse Values
        let mut values = vec![];
        while end < buffer.len() {
            let (row, row_end) = row_parser(buffer, end);
            values.push(row);
            if end == row_end {
                break;
            }
            end = row_end;
        }
        Self {
            header,
            default_value: default_values.first().unwrap().clone(),
            data_entries: DataEntry {
                data_entry_headers,
                values,
            },
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
        out.extend(to_u8_slice(&self.default_value));
        out.extend(vec_to_u8_slice(&self.data_entries.data_entry_headers));
        for row in &self.data_entries.values {
            out.extend(vec_to_u8_slice(row));
        }
        out
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct DataEntry {
    pub data_entry_headers: Vec<VariableCharArray>,
    pub values: Vec<Vec<VariableCharArray>>,
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
        let file = File::open("fixtures/xpcap.2da").unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Could not read to buffer");
        let item = TwoDA::new(&buffer);
        assert_eq!(
            item.header,
            Header {
                version: "V1.0".into(),
                signature: "2DA".into(),
            }
        );
        assert_eq!(item.default_value.to_string(), "2950000".to_string());
        assert_eq!(
            item.data_entries
                .data_entry_headers
                .first()
                .unwrap()
                .to_string(),
            "VALUE"
        );
        let last_values = item.data_entries.values.last().unwrap();
        assert_eq!(last_values.first().unwrap().to_string(), "SHAMAN");
        assert_eq!(last_values.last().unwrap().to_string(), "-1");
    }
}
