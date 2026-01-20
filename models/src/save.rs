use std::io::Read;
use std::{error::Error, path::Path};

use flate2::bufread::ZlibDecoder;
use serde::{Deserialize, Serialize};
use zerocopy::IntoBytes;
use zerovec::ule::VarULE;
use zerovec::{VarZeroVec, ZeroVec, make_varule};

use crate::{
    IEModels,
    common::{ZeroCharArray, types::ResourceType},
    from_buffer_with_resouce_type,
    model::Parseable,
};

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/sav_v1.htm
#[derive(Debug, Serialize, Deserialize)]
pub struct Save<'data> {
    pub signature: ZeroCharArray,
    pub version: ZeroCharArray,
    #[serde(borrow)]
    pub files: VarZeroVec<'data, SavedFileVARULE>,
}

impl<'data> Parseable<'data> for Save<'data> {}

impl<'data> TryFrom<&'data [u8]> for Save<'data> {
    type Error = Box<dyn Error>;

    fn try_from(value: &'data [u8]) -> Result<Self, Self::Error> {
        let signature = ZeroCharArray(value.get(0..4).unwrap_or_default().try_into()?);
        let version = ZeroCharArray(value.get(4..8).unwrap_or_default().try_into()?);
        let files = VarZeroVec::new();
        Ok(Self {
            signature,
            version,
            files,
        })
    }
}

impl<'data> TryInto<Vec<u8>> for Save<'data> {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut buffer = vec![];
        buffer.extend_from_slice(self.signature.as_bytes());
        buffer.extend_from_slice(self.version.as_bytes());
        for iefile in self.files.iter() {
            buffer.extend_from_slice(iefile.as_bytes());
        }
        Ok(buffer)
    }
}

// https://gibberlings3.github.io/iesdp/file_formats/ie_formats/sav_v1.htm#savv1_File

#[make_varule(SavedFileVARULE)]
#[zerovec::derive(Debug, Serialize, Deserialize)]
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct SavedFile<'data> {
    pub length_of_filename: u32,
    #[serde(borrow)]
    pub filename: &'data str,
    pub uncompressed_data_length: u32,
    pub compressed_data_length: u32,
    #[serde(borrow)]
    pub compressed_data: ZeroVec<'data, u8>,
}

impl<'data> SavedFile<'data> {
    pub fn decompress(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut d = ZlibDecoder::new(self.compressed_data.as_ule_slice());
        let mut buffer = vec![];
        d.read_to_end(&mut buffer).map_err(|err| {
            log::error!("{err}");
            err
        })?;
        Ok(buffer)
    }
}

pub fn parse_compressed_data<'data>(
    buffer: &'data [u8],
    file_path: &Path,
) -> Result<IEModels<'data>, Box<dyn Error>> {
    let resource_type = ResourceType::try_from(file_path)?;
    from_buffer_with_resouce_type(buffer, resource_type)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use std::{error::Error, fs};

    const FIXTURES: [(&str, &str); 1] = [("fixtures/baldur.sav", "fixtures/baldur.sav.json")];

    #[test]
    fn parse() -> Result<(), Box<dyn Error>> {
        for (file_path, json_file_path) in FIXTURES {
            let file = fs::read(file_path)?;
            let buffer = file.as_slice();
            let save: Save = Save::try_from(buffer)?;
            let result: Value = serde_json::to_value(save)?;
            let expected: Value = serde_json::from_slice(&fs::read(json_file_path)?)?;

            assert_eq!(result, expected);
        }
        Ok(())
    }
}
