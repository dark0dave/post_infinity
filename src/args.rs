use std::path::PathBuf;

use clap::{error::ErrorKind, ArgAction, Error, Parser};

use crate::writer::{as_binary, as_json, as_stdout, Printer};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Game lang
    #[clap(env, short = 'l', long, value_parser, default_value = "en_US")]
    pub game_lang: String,
    /// Flag to process tiles
    #[clap(env, long, short, action=ArgAction::SetTrue)]
    pub tiles: bool,
    /// Flag to process tlk file
    #[clap(env, long, short, action=ArgAction::SetTrue)]
    pub process_tlk: bool,
    /// Output Format, expects json(j), binary(b), print(p), or none(empty value)
    #[clap(env, long, short, value_parser = output_format_parser, default_value = "p")]
    pub output_format: Printer,
    /// Filename or prefix to extract [WARNING: EXPERIMENTAL]
    #[clap(env, long, short, default_value = "")]
    pub extract: String,
    /// Turn a json into an ie file type [WARNING: EXPERIMENTAL]
    #[clap(env, short='i', long, action=ArgAction::SetTrue)]
    pub to_ie_type: bool,
    /// If to_ie_type is set this controls the output
    #[clap(env, long, short, default_value = ".")]
    pub destination: PathBuf,
    /// The path of the file to read
    #[clap(env, long, short)]
    pub file: PathBuf,
    /// Flag to recurse through Keys or Biffs
    #[clap(env, long, short, action=ArgAction::SetTrue)]
    pub recurse: bool,
}

fn output_format_parser(input: &str) -> Result<Printer, clap::Error> {
    match input.to_lowercase().as_str() {
        "json" | "j" => Ok(as_json),
        "binary" | "bin" | "b" => Ok(as_binary),
        "" | "n" | "no" | "none" => Ok(|_, _, _| Ok(())),
        "p" | "print" => Ok(as_stdout),
        _ => Err(Error::new(ErrorKind::ValueValidation)),
    }
}
