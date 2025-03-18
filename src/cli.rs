use std::{error::Error, fs::File, io::Read, path::Path, rc::Rc, str};

use binrw::io::BufReader;
use models::{
    biff::Biff, common::types::ResourceType, from_buffer, from_json, key::Key, model::Model,
    tlk::TLK,
};

use crate::{
    args::Args,
    writer::{as_binary, as_json, to_stdout, write_file},
};

fn read_file(path: &Path) -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

fn json_back_to_ie_type(path: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
    let extension = path
        .extension()
        .ok_or(format!("Can't convert to str, {:?}", path))?
        .to_str()
        .ok_or(format!("Can't convert to str, {:?}", path))?
        .to_ascii_lowercase();

    let resource_type = ResourceType::try_from(path)?;
    let mut reader = read_file(path)?;
    let mut buffer = vec![];
    reader.read_to_end(&mut buffer)?;
    let out = from_json(&buffer, resource_type)?;
    let name = path.file_name().ok_or("Path has no file name")?;
    let out_path = dest.join(name);
    write_file(&out_path, &extension, &out)
}

fn get_models_from_file(
    path: &Path,
    output_format: &str,
    dest: &Path,
    recurse: bool,
) -> Result<(), Box<dyn Error>> {
    let resource_type = ResourceType::try_from(path)?;
    let mut reader: BufReader<File> = read_file(path)?;

    let model: Rc<dyn Model> = match resource_type {
        ResourceType::NotFound => {
            return Err(format!("Unprocessable file type: {:?}", path.as_os_str()).into());
        }
        ResourceType::FileTypeKey => {
            let mut buffer = vec![];
            reader.read_to_end(&mut buffer)?;
            let mut key = Key::new(&buffer);
            if recurse {
                key.recurse(path)?;
            }
            Rc::new(key)
        }
        ResourceType::FileTypeBiff => {
            let mut buffer = vec![];
            reader.read_to_end(&mut buffer)?;
            Rc::new(Biff::new(&buffer))
        }
        _ => {
            log::debug!("{:?}", resource_type);
            let mut buffer = vec![];
            reader.read_to_end(&mut buffer)?;
            from_buffer(&buffer, resource_type).ok_or("Could not parse file")?
        }
    };

    write(dest, resource_type, model, output_format)?;
    Ok(())
}

fn write(
    dest: &Path,
    resource_type: ResourceType,
    model: Rc<dyn Model>,
    output_format: &str,
) -> Result<(), Box<dyn Error>> {
    log::debug!("{}", output_format);
    match output_format {
        "json" => as_json(dest, model, resource_type),
        "binary" => as_binary(dest, model, resource_type),
        "print" => to_stdout(dest, model, resource_type),
        _ => Ok(()),
    }
}

pub fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    log::debug!("{:?}", args);
    let path = &args.file;
    get_models_from_file(path, &args.output_format, &args.destination, args.recurse)?;

    if args.to_ie_type {
        return json_back_to_ie_type(path, &args.destination);
    }

    if args.process_tlk {
        let game_directory = path.parent().ok_or("Could not find parent")?;
        let dialogue_path = game_directory
            .join("lang")
            .join(args.game_lang.clone())
            .join("dialog.tlk");
        let mut reader: BufReader<File> = read_file(&dialogue_path)?;
        let mut buffer = vec![];
        reader.read_to_end(&mut buffer)?;
        let static_buffer: &'static [u8] = Box::leak(buffer.into_boxed_slice());
        TLK::parse(static_buffer).ok_or("Could not parse TLK file")?;
    }
    Ok(())
}
