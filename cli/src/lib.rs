pub mod args;
use std::{
    fs::{self, File},
    io::{self, BufReader, Read, Write},
    path::Path,
    process::exit,
    str,
};

use args::Args;
use models::{
    biff::Biff,
    from_buffer,
    key::Key,
    model::Model,
    resources::types::{extension_to_resource_type, ResourceType},
    save::Save,
    spell::Spell,
    tlk::Lookup,
};

use erased_serde::Serializer;

fn write_file(path: &Path, extension: &str, buffer: &[u8]) {
    let file_name = Path::new(path.file_stem().unwrap_or_default()).with_extension(extension);
    if let Ok(mut file) = File::create(file_name) {
        if let Err(err) = file.write_all(buffer) {
            println!("Error: {}", err);
        }
    }
}

fn json_back_to_ie_type(path: &Path) {
    let file_contents = read_file(path);
    if let Ok(spell) = serde_json::from_slice::<Spell>(&file_contents) {
        write_file(path, "spl", &spell.to_bytes())
    } else {
        panic!("Could not convert back to model")
    }
}

fn write_model(path: &Path, model: std::rc::Rc<dyn Model>, resource_type: ResourceType) {
    let file_name = Path::new(path.file_stem().unwrap_or_default())
        .with_extension(format!("{}.json", resource_type));
    println!("{:?}", file_name);
    if let Ok(file) = File::create(file_name) {
        let mut json = serde_json::Serializer::new(file);
        let mut format = <dyn Serializer>::erase(&mut json);
        if let Err(err) = model.erased_serialize(&mut format) {
            panic!("{}", err);
        }
    }
}

fn read_file(path: &Path) -> Vec<u8> {
    let file = File::open(path).unwrap_or_else(|_| panic!("Could not open file: {:#?}", path));
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader
        .read_to_end(&mut buffer)
        .expect("Could not read to buffer");
    buffer
}

fn parse_tlk_file(path: &Path) -> Lookup {
    let buffer = read_file(path);
    Lookup::new(&buffer)
}

fn parse_key_file(path: &Path, buffer: &[u8]) -> Vec<Biff> {
    let key: Key = Key::new(buffer);
    let parent = path.parent().unwrap();

    key.bif_entries
        .iter()
        .map(|bif_ref| {
            let buffer = read_file(&parent.join(bif_ref.name.to_string()).with_extension("bif"));
            Biff::new(&buffer)
        })
        .collect()
}

fn get_model_from_file(path: &Path, json: bool) -> Vec<Biff> {
    let buffer = read_file(path);
    let extention = path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_ascii_lowercase();
    let resource_type = extension_to_resource_type(&extention);

    // Non resource types
    if resource_type == ResourceType::NotFound {
        return match extention.as_str() {
            "key" => parse_key_file(path, &buffer),
            "biff" => vec![Biff::new(&read_file(path))],
            "sav" => {
                let mut save = Save::new(&buffer);
                save.decompress();
                println!("{:#?}", save);
                exit(0)
            }
            "json" => {
                json_back_to_ie_type(path);
                exit(0)
            }
            _ => panic!("Unprocessable file type: {:?}", path.as_os_str()),
        };
    }

    let model = from_buffer(&buffer, resource_type).expect("Could not parse file");
    if json {
        write_model(path, model, resource_type);
    } else {
        let print = &mut serde_json::Serializer::new(io::stdout());
        let mut format = <dyn Serializer>::erase(print);
        model.erased_serialize(&mut format).unwrap();
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
                get_model_from_file(&file, args.json)
            })
            .collect()
    } else {
        get_model_from_file(dir_or_file, args.json)
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
