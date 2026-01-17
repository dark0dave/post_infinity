use std::{error::Error, fs::File, io::Write, path::Path, str};

use models::{IEModels, common::types::ResourceType};

pub(crate) type Printer = fn(&Path, IEModels, ResourceType) -> Result<(), Box<dyn Error>>;

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

pub(crate) fn as_stdout(_: &Path, model: IEModels, _: ResourceType) -> Result<(), Box<dyn Error>> {
    println!("{}", model.to_json()?);
    Ok(())
}

pub(crate) fn as_binary(
    dest: &Path,
    model: IEModels,
    _: ResourceType,
) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(dest)?;
    let bytes = model.to_bytes()?;
    Ok(file.write_all(&bytes)?)
}

pub(crate) fn as_json(
    dest: &Path,
    model: IEModels,
    resource_type: ResourceType,
) -> Result<(), Box<dyn Error>> {
    let extension: String = resource_type.into();
    let file_name =
        Path::new(dest.file_stem().unwrap_or_default()).with_extension(format!("{extension}.json"));
    log::info!("Saved as {file_name:#?}");

    let file = File::create(file_name)?;
    Ok(serde_json::to_writer(file, &model.to_json()?)?)
}
