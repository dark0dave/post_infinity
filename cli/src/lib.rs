pub mod args;
use std::{
    fs::{self, File},
    io::{BufReader, Read},
    path::Path,
    process::exit,
};

use args::Args;
use models::{
    biff::Biff,
    from_buffer,
    key::Key,
    resources::types::{extention_to_resource_type, ResourceType},
    tlk::Lookup,
};

use erased_serde::Serializer;

fn from_file(path: &Path) -> Vec<u8> {
    let file = File::open(path).unwrap_or_else(|_| panic!("Could not open file: {:#?}", path));
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .expect("Could not read to buffer");
    buffer
}

fn parse_tlk_file(path: &Path) -> Lookup {
    let buffer = from_file(path);
    Lookup::new(&buffer)
}

fn parse_key_file(path: &Path, buffer: &[u8]) -> Vec<Biff> {
    let key: Key = Key::new(buffer);
    let parent = path.parent().unwrap();

    key.bif_entries
        .iter()
        .map(|bif_ref| {
            let buffer = from_file(&parent.join(bif_ref.name.to_string()).with_extension("bif"));
            Biff::new(&buffer)
        })
        .collect()
}

fn get_model_from_file(path: &Path) -> Vec<Biff> {
    let buffer = from_file(path);
    let extention = path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_ascii_lowercase();
    let resource_type = extention_to_resource_type(&extention);

    // Non resource types
    if resource_type == ResourceType::NotFound {
        return match extention.as_str() {
            "key" => parse_key_file(path, &buffer),
            "biff" => vec![Biff::new(&from_file(path))],
            "json" => panic!(),
            _ => panic!("Unprocessable file type: {:?}", path.as_os_str()),
        };
    }

    let buffer = from_buffer(&buffer, resource_type).expect("Could not parse file");
    let file_name = Path::new(path.file_stem().unwrap_or_default()).with_extension("json");
    if let Ok(file) = File::create(file_name) {
        let mut json = serde_json::Serializer::new(file);
        let mut format = <dyn Serializer>::erase(&mut json);
        if let Err(err) = buffer.erased_serialize(&mut format) {
            panic!("{}", err);
        }
    }
    exit(0)
}

pub fn read_files(args: &Args) -> (Vec<Biff>, Option<Lookup>) {
    let dir_or_file = &args.resource_file_or_dir;

    let biffs = if dir_or_file.is_dir() {
        let paths = fs::read_dir(dir_or_file).expect("Could not read files in directory");
        paths
            .into_iter()
            .flat_map(|path| {
                let file = path.unwrap().path();
                get_model_from_file(&file)
            })
            .collect()
    } else {
        get_model_from_file(dir_or_file)
    };

    let lookup = match args.process_tlk {
        true if dir_or_file.parent().is_some() => {
            let game_directory = dir_or_file.parent().unwrap();
            let tlk_path = game_directory
                .join("lang")
                .join(args.game_lang.clone())
                .join("dialog.tlk");
            Some(parse_tlk_file(&tlk_path))
        }
        _ => None,
    };

    (biffs, lookup)
}
