#![allow(unused)]
use std::fmt::Debug;
use std::fs;
use std::path::{PathBuf};
use anyhow::{Result};
use clap::{ValueEnum, Parser};
use chrono::{DateTime, Utc};

/// only allow explicit values and assign an extension type for each
#[derive(Debug, Clone, ValueEnum)]
enum ArchiveType {
    Tar,
    TarGunzip,
    Zip,
}

impl ArchiveType {
    fn as_str(&self) -> &'static str {
        match self {
            ArchiveType::Tar => "tar",
            ArchiveType::TarGunzip => "tar.gz",
            ArchiveType::Zip => "zip"
        }
    }
}

/// Self-explanatory
fn get_file_mtime(file: &str) -> Result<i64> {
    let _file_metadata: DateTime <Utc> = fs::metadata(file.to_string())?
        .modified()?
        .into();

    let now: DateTime<Utc> = Utc::now();
    let diff: i64 = now
        .signed_duration_since(_file_metadata)
        .num_days();

    Ok(diff)
}

/// Boilerplate for future function that checks mtime diff 
/// and archives / removes if a threshold is met
fn archive_file(file: &str, threshold_days: i64, archive_type: ArchiveType) -> Result<()> {
    let _mtime_diff = get_file_mtime(file)?;
    if _mtime_diff > threshold_days {
        println!("Archive Type: {}", archive_type.as_str());
        println!("File Path: {}", file);
    }

    Ok(())
}

/// Create a vector to store all *unfiltered* files in the provided directory
fn gather_files_from_directory(dir_path: &str) -> Result<Vec<PathBuf>> {
    let files: Vec<PathBuf> = fs::read_dir(dir_path.to_string())?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    Ok(files)
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
    let _test_file = "/tmp/test_logs/test_file_16.log.tar.gz";
    let _test_diff = get_file_mtime(_test_file)?;

    println!("Archive Method: {}", args.archive_method.as_str());
    println!("Directory: {:?}", args.directory);
    println!("Difference in file mtime and current date: {}", _test_diff);
    println!("Files in provided directory: {:?}", gather_files_from_directory(&args.directory)?);

    archive_file(_test_file, 1, ArchiveType::Tar).expect("Failed to archive file");

    Ok(())
}
