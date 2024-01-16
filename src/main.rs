#![feature(fs_try_exists)]

mod rusky;

use clap::{Parser, Subcommand};
use crate::Commands::Setup;
use rusky::{rusky_default, rusky_setup};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initial setup of rusky
    Setup,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Setup) => rusky_setup(),
        None => rusky_default()
    }
}
