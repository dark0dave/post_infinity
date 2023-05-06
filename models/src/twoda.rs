use std::{rc::Rc, vec};

use std::fmt::Debug;

use serde::Serialize;

use crate::common::fixed_char_array::FixedCharSlice;
use crate::common::header::Header;
use crate::common::varriable_char_array::{VarriableCharArray, DEFAULT};
use crate::model::Model;
use crate::resources::utils::row_parser;
use crate::tlk::Lookup;

//https://gibberlings3.github.io/iesdp/file_formats/ie_formats/2da.htm
#[derive(Debug, Serialize)]
pub struct TwoDA {
    pub header: Header<3, 4>,
    pub default_value: VarriableCharArray,
    pub data_entries: DataEntry,
}

#[derive(Debug, Serialize)]
pub struct DataEntry {
    pub data_entry_headers: Vec<VarriableCharArray>,
    pub values: Vec<Vec<VarriableCharArray>>,
}

impl Model for TwoDA {
    fn new(buffer: &[u8]) -> Self {
        // Parse Headers
        let (headers, end) = row_parser(buffer, 0);

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
            header: Header {
                signature: FixedCharSlice::<3>::try_from(headers.first().unwrap_or(DEFAULT))
                    .unwrap_or_else(|_| "2DA".into()),
                version: FixedCharSlice::<4>::try_from(headers.last().unwrap_or(DEFAULT))
                    .unwrap_or_default(),
            },
            default_value: default_values.first().unwrap().clone(),
            data_entries: DataEntry {
                data_entry_headers,
                values,
            },
        }
    }

    fn create_as_rc(buffer: &[u8]) -> std::rc::Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }

    fn name(&self, _lookup: &Lookup) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use crate::common::varriable_char_array::DEFAULT;

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
                .unwrap_or(DEFAULT)
                .to_string(),
            "VALUE"
        );
        let last_values = item.data_entries.values.last().unwrap();
        assert_eq!(last_values.first().unwrap_or(DEFAULT).to_string(), "SHAMAN");
        assert_eq!(last_values.last().unwrap_or(DEFAULT).to_string(), "-1");
    }
}
