use logrotate::{ArchiveType, archive_file, gather_files_from_directory, get_file_mtime_diff};

use anyhow::{Result};
use std::fmt::Debug;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Perform a dry run without making any changes
    /// Will output files marked for deletion, archival, and truncation
    #[arg(
        long
    )]
    dry_run: bool,

    /// Archival method to use
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
    let _test_file = "/tmp/test_logs/test_file_16.log.tar.gz";
    let _test_dir = "/tmp/test_logs";
    let _test_diff = get_file_mtime_diff(_test_file)?;

    println!("Provided ARGS Archive Method: {:?}", args.archive_method);
    println!("Provided ARGS Directory: {:?}", args.directory);
    println!("Difference in file mtime and current date: {:?}", _test_diff);
    println!("Files in provided directory: {:?}", gather_files_from_directory(&_test_dir)?);

    archive_file(_test_file, 1, ArchiveType::Tar).expect("Failed to archive file");

    Ok(())
}
