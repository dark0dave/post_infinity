use std::{
    fs::{self, File},
    io::{BufReader, Read},
    path::Path,
};

use models::{
    biff::Biff,
    key::Key,
    model::Model,
    resources::types::{extention_to_resource_type, ResourceType},
    utils::from_buffer,
};

pub fn from_file(path: &Path) -> Vec<u8> {
    let file = File::open(path).unwrap_or_else(|_| panic!("Could not open file: {:#?}", path));
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .expect("Could not read to buffer");
    buffer
}

fn parse_key_file(path: &Path, process_tiles: bool, buffer: &[u8]) {
    let key: Key = Key::new(buffer);
    let parent = path.parent().unwrap();

    let bifs: Vec<Biff> = key
        .bif_entries
        .iter()
        .map(|bif_ref| {
            let buffer = from_file(&parent.join(bif_ref.name.to_string()).with_extension("bif"));
            Biff::new(&buffer, process_tiles)
        })
        .collect();
    println!("Processed {:#?}", bifs);
}

fn get_model_from_file(path: &Path, process_tiles: bool) {
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
        match extention.as_str() {
            "key" => parse_key_file(path, process_tiles, &buffer),
            "biff" => todo!(),
            _ => panic!("Unprocessable file type: {:?}", path.as_os_str()),
        };
    } else {
        println!("Processed {:#?}", from_buffer(&buffer, resource_type));
    }
}

pub fn read_files(dir_or_file: &Path, process_tiles: bool) {
    if dir_or_file.is_dir() {
        let paths = fs::read_dir(dir_or_file).unwrap();

        for path in paths {
            let file = path.unwrap().path();
            get_model_from_file(&file, process_tiles);
        }
    } else {
        get_model_from_file(dir_or_file, process_tiles);
    }
}
