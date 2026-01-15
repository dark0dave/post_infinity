use std::{error::Error, fs::File, io::Write, path::Path, str};

use erased_serde::Serializer;
use models::{IEModel, common::types::ResourceType};

pub(crate) type Printer = fn(&Path, IEModel, ResourceType) -> Result<(), Box<dyn Error>>;

pub(crate) fn write_file(
    path: &Path,
    extension: &str,
    buffer: &[u8],
) -> Result<(), Box<dyn Error>> {
    let file_name = Path::new(path.file_stem().unwrap_or_default()).with_extension(extension);
    if let Ok(mut file) = File::create(file_name) {
        return Ok(file.write_all(buffer)?);
    }
    Ok(())
}

pub(crate) fn as_stdout(_: &Path, model: IEModel, _: ResourceType) -> Result<(), Box<dyn Error>> {
    let print = &mut serde_json::Serializer::new(std::io::stdout());
    let mut format = <dyn Serializer>::erase(print);
    Ok(model.erased_serialize(&mut format)?)
}

pub(crate) fn as_binary(
    dest: &Path,
    model: IEModel,
    _: ResourceType,
) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(dest)?;
    let bytes = model.to_bytes();
    Ok(file.write_all(&bytes)?)
}

pub(crate) fn as_json(
    dest: &Path,
    model: IEModel,
    resource_type: ResourceType,
) -> Result<(), Box<dyn Error>> {
    let extension: String = resource_type.into();
    let file_name =
        Path::new(dest.file_stem().unwrap_or_default()).with_extension(format!("{extension}.json"));
    log::info!("Saved as {file_name:#?}");

    let file = File::create(file_name)?;
    let mut json = serde_json::Serializer::new(file);
    let mut format = <dyn Serializer>::erase(&mut json);
    Ok(model.erased_serialize(&mut format)?)
}
