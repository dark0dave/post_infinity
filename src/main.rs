use std::process::ExitCode;

use clap::Parser;

use args::Args;
use env_logger::Env;

pub mod args;
pub mod cli;
pub mod writer;

fn main() -> ExitCode {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args: Args = Args::parse();
    match cli::run(&args) {
        Err(err) => {
            log::error!("{:?}", err);
            ExitCode::FAILURE
        }
        _ => ExitCode::SUCCESS,
    }
}
