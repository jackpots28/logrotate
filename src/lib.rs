use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use clap::ValueEnum;

/// only allow explicit values and assign an extension type for each
/// this is used to only allow specific archive types as flags for cli
#[derive(Debug, Clone, ValueEnum)]
pub enum ArchiveType {
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
pub fn get_file_mtime_diff(file: &str) -> anyhow::Result<i64> {
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
pub fn archive_file(file: &str, threshold_days: i64, archive_type: ArchiveType) -> anyhow::Result<()> {
    let _mtime_diff = get_file_mtime_diff(file)?;
    if _mtime_diff > threshold_days {
        println!("Archive Type: {}", archive_type.as_str());
        println!("File Path: {}", file);
    }

    Ok(())
}

/// Create a vector to store all *unfiltered* files in the provided directory
pub fn gather_files_from_directory(dir_path: &str) -> anyhow::Result<Vec<PathBuf>> {
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

/// Do not worry about testing this function - only renders a file list to stdout
pub fn dry_run_details(file_list: Vec<PathBuf>) {
    for file in file_list {
        println!("File: {}", file.to_str().unwrap());
    }
}


/// Fake test function
pub fn test_add(left: u64, right: u64) -> u64 {
    left + right
}