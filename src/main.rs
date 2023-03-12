use clap::Parser;
use cli::args::Args;

fn main() {
    let args: Args = Args::parse();
    cli::read_files(&args);
}
