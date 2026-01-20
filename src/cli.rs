use std::{error::Error, fs, path::Path};

use models::{common::types::ResourceType, from_buffer_with_resouce_type, from_json, tlk::TLK};

use crate::{
    args::Args,
    writer::{Printer, write_file},
};

fn json_back_to_ie_type(path: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
    let extension = path
        .extension()
        .ok_or(format!("Can't convert to str, {path:?}"))?
        .to_str()
        .ok_or(format!("Can't convert to str, {path:?}"))?
        .to_ascii_lowercase();

    let resource_type = ResourceType::try_from(path)?;
    let buffer = fs::read(path)?;
    let out = from_json(&buffer, resource_type)?;
    let name = path.file_name().ok_or("Path has no file name")?;
    let out_path = dest.join(name);
    write_file(&out_path, &extension, &out)
}

fn get_models_from_file(path: &Path, printer: Printer, dest: &Path) -> Result<(), Box<dyn Error>> {
    let resource_type = ResourceType::try_from(path)?;
    if resource_type == ResourceType::NotFound {
        return Err(format!("Unprocessable file type: {:?}", path.as_os_str()).into());
    }
    let buffer = fs::read(path)?;
    let static_buffer: &'static [u8] = Box::leak(buffer.into_boxed_slice());
    let model = from_buffer_with_resouce_type(static_buffer, resource_type)?;

    printer(dest, model, resource_type)
}

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    log::debug!("{args:?}");
    let path = &args.file;
    get_models_from_file(path, args.output_format, &args.destination)?;

    if args.to_ie_type {
        return json_back_to_ie_type(path, &args.destination);
    }

    if args.process_tlk {
        let game_directory = path.parent().ok_or("Could not find parent")?;
        let dialogue_path = game_directory
            .join("lang")
            .join(args.game_lang.clone())
            .join("dialog.tlk");
        let buffer = fs::read(&dialogue_path)?;
        TLK::try_from(buffer.as_slice())?;
    }
    Ok(())
}
