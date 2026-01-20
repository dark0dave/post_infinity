use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::model::Parseable;

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/ids.htm
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[repr(C, packed)]
pub struct Ids<'data> {
    #[serde(borrow)]
    pub data: &'data str,
}

impl<'data> Parseable<'data> for Ids<'data> {}

impl<'data> TryFrom<&'data [u8]> for Ids<'data> {
    type Error = Box<dyn Error>;

    fn try_from(value: &'data [u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            data: unsafe { (value as *const _ as *mut [u8] as *mut str).as_ref() }
                .ok_or("Could not convert from bytes")?,
        })
    }
}

impl<'data> TryInto<Vec<u8>> for Ids<'data> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Ok(unsafe { (self.data as *const _ as *const [u8]).as_ref() }
            .ok_or("Could not convert to string")?
            .to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use std::error::Error;
    use std::fs;

    const FIXTURES: [(&str, &str); 1] = [("fixtures/race.ids", "fixtures/race.ids.json")];

    #[test]
    fn parse() -> Result<(), Box<dyn Error>> {
        for (file_path, json_file_path) in FIXTURES {
            let buffer = fs::read(file_path)?;
            let ie: Ids = Ids::try_from(buffer.as_slice())?;
            let result: Value = serde_json::to_value(&ie)?;
            let expected: Value = serde_json::from_slice(&fs::read(json_file_path)?)?;

            assert_eq!(result, expected);
        }
        Ok(())
    }
}
