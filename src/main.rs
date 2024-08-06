use clap::Parser;

use args::Args;

pub mod args;
pub mod cli;

fn main() {
    let args: Args = Args::parse();
    cli::read_files(&args);
}
