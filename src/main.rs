#![allow(unused_imports)]

use logrotate::{
    ArchiveType,
    archive_or_remove_file,
    gather_files_from_directory,
    get_file_mtime_diff,
    dry_run_details
};

use anyhow::{Result};
use std::fmt::Debug;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Rotate / Archive files within provided directory", long_about = None)]
struct Cli {
    /// Perform a dry run without making any changes
    /// Will output files marked for deletion, archival, and truncation
    #[arg(
        long,
        required = false,
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
    
    /// Number of days to keep archived files
    #[arg(
        short = 'k',
        long = "keep-days",
        value_name = "DAYS",
        default_value = "7",
        required = true,
    )]
    keep_days: u8,
}

#[cfg(not(tarpaulin_include))]
fn main() -> Result<()> {
    // Bug with Clap Derive - False error: 
    // E0599 No function or associated item 'parse' found in the current scope for struct Cli
    let args = <Cli as Parser>::parse();

    let arg_directory = args.directory;
    let arg_archive_method = args.archive_method;
    let arg_keep_days = args.keep_days;

    let file_list = gather_files_from_directory(&arg_directory)?;
    
    if args.dry_run {
        println!("Dry Run with following args\n\
         ARCHIVE METHOD: {:?}\n\
         DIRECTORY PATH: {:?}\n\
         KEEP FOR: {:?} DAYS", 
                 arg_archive_method, arg_directory, arg_keep_days
        );
        
        dry_run_details(file_list, arg_keep_days.into(), arg_archive_method);
        
        // Early Exit
        return Ok(());   
    }

    // archive_file(_test_file, 1, ArchiveType::Tar).expect("Failed to archive file");

    Ok(())
}
