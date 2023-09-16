use std::rc::Rc;

use serde::Serialize;

use crate::{
    common::{
        fixed_char_array::FixedCharSlice, header::Header, varriable_char_array::VarriableCharArray,
    },
    model::Model,
    resources::utils::row_parser,
    tlk::Lookup,
};

//https://gibberlings3.github.io/iesdp/file_formats/ie_formats/ids.htm
#[derive(Debug, Serialize)]
pub struct Ids {
    pub header: Header<3, 4>,
    pub data_entries: Vec<DataEntry>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct DataEntry {
    pub value: VarriableCharArray,
    pub identifier: VarriableCharArray,
}

impl Model for Ids {
    fn new(buffer: &[u8]) -> Self {
        let (headers, mut end) = row_parser(buffer, 0);

        let signature = if let Some(_) = headers.first() {
            FixedCharSlice::<3>::from(&buffer[0..3])
        } else {
            FixedCharSlice::<3>::default()
        };
        let version = match headers.last() {
            Some(x) if FixedCharSlice::<4>::try_from(x).is_ok() => {
                FixedCharSlice::<4>::try_from(x).unwrap()
            }
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

    fn create_as_rc(buffer: &[u8]) -> std::rc::Rc<dyn Model> {
        Rc::new(Self::new(buffer))
    }

    fn name(&self, _lookup: &Lookup) -> String {
        todo!()
    }
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
