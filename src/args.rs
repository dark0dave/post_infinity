use std::path::PathBuf;

use clap::{error::ErrorKind, ArgAction, Error, Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Game lang
    #[clap(env, short = 'l', long, value_parser, default_value = "en_US")]
    pub game_lang: String,
    /// Flag to process tiles
    #[clap(env, long, short, action=ArgAction::SetTrue)]
    pub tiles: bool,
    /// Flag to tlk file
    #[clap(env, long, short, action=ArgAction::SetTrue)]
    pub process_tlk: bool,
    /// Output Format, expects json(j), binary(b), print(p), or none(empty value)
    #[clap(env, long, short, value_parser=output_format_parser, default_value = "")]
    pub output_format: String,
    /// Filename or prefix to extract [WARNING: EXPERIMENTAL]
    #[clap(env, long, short, default_value = "")]
    pub extract: String,
    /// Turn a json into an ie file type [WARNING: EXPERIMENTAL]
    #[clap(env, short='i', long, action=ArgAction::SetTrue)]
    pub to_ie_type: bool,
    /// Read Save game
    #[clap(env, long, short, action=ArgAction::SetTrue)]
    pub save: bool,
    /// Read Save game
    #[clap(env, long, short, default_value = ".")]
    pub destination: PathBuf,
    /// The path to the file to read
    #[clap(env, long, short)]
    pub resource_file_or_dir: PathBuf,
}

fn output_format_parser(input: &str) -> Result<String, clap::Error> {
    match input.to_lowercase().as_str() {
        "json" | "j" => Ok("json".to_string()),
        "binary" | "bin" | "b" => Ok("binary".to_string()),
        "" | "n" | "no" | "none" => Ok("".to_string()),
        "p" | "print" => Ok("print".to_string()),
        _ => Err(Error::new(ErrorKind::ValueValidation)),
    }
}
