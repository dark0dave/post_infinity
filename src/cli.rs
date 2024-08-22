use std::{fs::File, path::Path, process::exit, str};

use binrw::io::{BufReader, Write};
use models::{
    biff::Biff, common::types::ResourceType, from_buffer, from_json, key::Key, model::Model,
    save::Save, tlk::TLK,
};

use erased_serde::Serializer;

use crate::args::Args;

fn write_file(path: &Path, extension: &str, buffer: &[u8]) {
    let file_name = Path::new(path.file_stem().unwrap_or_default()).with_extension(extension);
    if let Ok(mut file) = File::create(file_name) {
        if let Err(err) = file.write_all(buffer) {
            println!("Error: {}", err);
        }
    }
}

fn json_back_to_ie_type(path: &Path) {
    let extension = path
        .extension()
        .unwrap_or_default()
        .to_ascii_lowercase()
        .into_string()
        .unwrap_or_default();

    let resource_type = ResourceType::from(extension.as_str());
    let file_reader = read_file(path);
    let out = from_json(file_reader.buffer(), resource_type);
    write_file(path, &extension, &out);
}

fn write_model(path: &Path, model: std::rc::Rc<dyn Model>, resource_type: ResourceType) {
    let file_name = Path::new(path.file_stem().unwrap_or_default())
        .with_extension(format!("{}.json", resource_type));
    println!("Saved as {:#?}", file_name);
    if let Ok(file) = File::create(file_name) {
        let mut json = serde_json::Serializer::new(file);
        let mut format = <dyn Serializer>::erase(&mut json);
        if let Err(err) = model.erased_serialize(&mut format) {
            panic!("{}", err);
        }
    }
}

fn read_file(path: &Path) -> BufReader<File> {
    let file = File::open(path).unwrap_or_else(|_| panic!("Could not open file: {:#?}", path));

    BufReader::new(file)
}

fn parse_key_file(path: &Path, reader: &mut BufReader<File>) -> Vec<Biff> {
    let key: Key = Key::new(reader);
    let parent = path.parent().unwrap();

    key.bif_file_names
        .iter()
        .map(|file_name| {
            let mut reader = read_file(&parent.join(file_name.to_string().replace('\0', "")));
            Biff::new(&mut reader)
        })
        .collect()
}

fn get_model_from_file(path: &Path, json: bool) -> Vec<Biff> {
    let mut reader = read_file(path);
    let extention = path
        .extension()
        .unwrap_or_default()
        .to_ascii_lowercase()
        .into_string()
        .unwrap_or_default();

    match ResourceType::from(extention.as_str()) {
        ResourceType::NotFound => {
            return match extention.as_str() {
                "key" => parse_key_file(path, &mut reader),
                "biff" => vec![Biff::new(&mut reader)],
                "sav" => {
                    for file in Save::new(&mut reader).files {
                        println!("{:#?}", file.uncompressed_data);
                    }
                    exit(0)
                }
                "json" => {
                    json_back_to_ie_type(path);
                    exit(0)
                }
                _ => panic!("Unprocessable file type: {:?}", path.as_os_str()),
            };
        }
        resource_type => {
            let model = from_buffer(reader.buffer(), resource_type).expect("Could not parse file");
            if json {
                write_model(path, model, resource_type);
            } else {
                let print = &mut serde_json::Serializer::new(std::io::stdout());
                let mut format = <dyn Serializer>::erase(print);
                model.erased_serialize(&mut format).unwrap();
            }
        }
    }
    exit(0)
}

pub fn read_files(args: &Args) -> (Vec<Biff>, Option<TLK>) {
    let dir_or_file = &args.resource_file_or_dir;

    let biffs = if dir_or_file.is_dir() {
        let paths = std::fs::read_dir(dir_or_file).expect("Could not read files in directory");
        paths
            .into_iter()
            .flat_map(|path| {
                let file = path.unwrap().path();
                get_model_from_file(&file, args.json)
            })
            .collect()
    } else {
        get_model_from_file(dir_or_file, args.json)
    };

    let tlk = match args.process_tlk {
        true if dir_or_file.parent().is_some() => {
            let game_directory = dir_or_file.parent().unwrap();
            let path = game_directory
                .join("lang")
                .join(args.game_lang.clone())
                .join("dialog.tlk");
            let mut reader = read_file(&path);
            Some(TLK::new(&mut reader))
        }
        _ => None,
    };

    (biffs, tlk)
}
