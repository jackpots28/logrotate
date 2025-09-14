use std::io::prelude::*;
use std::fs::File;
use anyhow::Result;
use clap::{Arg, Command, ValueEnum, Parser};
use log::{info, warn};

#[derive(Debug, Clone, ValueEnum)]
enum ArchiveType {
    Tar,
    TarGunzip,
    Zip,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Perform a dry run without making any changes
    #[arg(
        long
    )]
    dry_run: bool,

    /// Archive method to use
    #[arg(
        short = 'a',
        long = "archive-method",
        value_enum,
        required = true,
    )]
    archive_method: ArchiveType,

    /// Directory to parse through
    #[arg(
        short = 'd',
        long = "directory",
        value_name = "DIRECTORY",
        required = true,
    )]
    directory: String,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    info!("-- Testing --");
    println!("Archive Method: {:?}", args.archive_method);
    println!("Directory: {:?}", args.directory);
    Ok(())
}
