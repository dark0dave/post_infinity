use std::{error::Error, fs::File, io::Read, path::Path, rc::Rc, str};

use binrw::io::BufReader;
use models::{
    biff::Biff, common::types::ResourceType, from_buffer, from_json, key::Key, model::Model,
    save::Save, tlk::TLK,
};

use crate::{
    args::Args,
    writer::{as_binary, as_json, to_stdout, write_file},
};

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
    let out = from_json(&buffer, resource_type);
    let name = path.file_name().ok_or("Path has no file name")?;
    let out_path = dest.join(name);
    write_file(&out_path, &extension, &out)
}

fn read_file(path: &Path) -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

fn parse_key_file(path: &Path, reader: &mut BufReader<File>) -> Result<Vec<Biff>, Box<dyn Error>> {
    let key: Key = Key::new(reader);
    let parent = path.parent().unwrap();
    let mut out = vec![];
    for bif_file_name in key.bif_file_names {
        let file_path = &parent.join(bif_file_name.to_string().replace('\0', ""));
        let mut reader = read_file(file_path)?;
        out.push(Biff::new(&mut reader))
    }
    Ok(out)
}

fn get_models_from_file(
    path: &Path,
    output_format: &str,
    dest: &Path,
) -> Result<(), Box<dyn Error>> {
    let resource_type = ResourceType::try_from(path)?;
    let mut reader: BufReader<File> = read_file(path)?;

    if resource_type == ResourceType::NotFound {
        let extension = path
            .extension()
            .ok_or(format!("Can't convert to str, {:?}", path))?
            .to_str()
            .ok_or(format!("Can't convert to str, {:?}", path))?
            .to_ascii_lowercase();
        log::debug!("{}", extension);
        let biffs = match extension.as_str() {
            "key" => parse_key_file(path, &mut reader)?,
            "biff" => vec![Biff::new(&mut reader)],
            _ => return Err(format!("Unprocessable file type: {:?}", path.as_os_str()).into()),
        };
        for biff in biffs {
            for model in biff.contained_files {
                process(dest, resource_type, model, output_format)?;
            }
        }
        return Ok(());
    }

    let mut buffer = vec![];
    reader.read_to_end(&mut buffer)?;
    let model = from_buffer(&buffer, resource_type).ok_or("Could not parse file")?;
    process(dest, resource_type, model, output_format)?;
    Ok(())
}

fn process(
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
    let path = &args.resource_file_or_dir;
    let paths = if path.is_dir() {
        let paths = std::fs::read_dir(path)?;
        paths.into_iter().map(|path| path.unwrap().path()).collect()
    } else {
        vec![path.clone()]
    };

    if args.save {
        let mut reader: BufReader<File> = read_file(path)?;
        for file in Save::new(&mut reader).files {
            log::info!("{:#?}", file.uncompressed_data);
        }
        return Ok(());
    }

    if args.to_ie_type {
        for path in paths {
            json_back_to_ie_type(&path, &args.destination)?
        }
        return Ok(());
    }

    for path in paths {
        let name = path.file_name().ok_or("Path has no file name")?;
        let dest = &args.destination.clone().join(name);
        get_models_from_file(&path, &args.output_format, dest)?;
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
        TLK::parse(&buffer).ok_or("Could not parse TLK file")?;
    }
    Ok(())
}
