use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Game lang
    #[clap(long, value_parser, default_value = "en_US")]
    pub game_lang: String,
    /// Flag to process tiles
    #[clap(long, short, action=ArgAction::SetTrue)]
    pub tiles: bool,
    /// Flag to tlk file
    #[clap(long, short, action=ArgAction::SetFalse)]
    pub process_tlk: bool,
    /// Flag to tlk file
    #[clap(long, short, action=ArgAction::SetTrue)]
    pub json: bool,
    /// The path to the file to read
    pub resource_file_or_dir: std::path::PathBuf,
}
