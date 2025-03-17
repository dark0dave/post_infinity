use std::io::{Read, Seek};

use binrw::BinResult;

#[binrw::writer(writer)]
#[allow(clippy::all)]
pub fn write_string(contents: &String) -> Result<(), binrw::Error> {
    let _ = writer.write(&contents.clone().into_bytes())?;
    Ok(())
}

#[binrw::parser(reader)]
pub fn read_to_end() -> BinResult<String> {
    let mut buff = String::new();
    reader.read_to_string(&mut buff).unwrap_or_default();
    Ok(buff)
}

pub fn read_string<R: Read + Seek>(reader: &mut R, limit: u64) -> BinResult<String> {
    let mut buff = String::new();
    reader.take(limit).read_to_string(&mut buff)?;
    Ok(buff)
}
