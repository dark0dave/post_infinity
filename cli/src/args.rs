use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The path to the file to read
    pub resource_file_or_dir: std::path::PathBuf,
    /// Flag to process tiles
    #[clap(long, value_parser, default_value = "false")]
    pub process_tiles: bool,
}
