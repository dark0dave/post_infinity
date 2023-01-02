use args::Args;
use clap::Parser;

mod args;
mod cli;

fn main() {
    let args: Args = Args::parse();
    cli::read_files(&args.resource_file_or_dir, args.process_tiles);
}
