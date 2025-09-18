// old - use strum_macros::Display;

use std::fmt;
use std::fs;
use std::path;
use tar::Builder;
use flate2::Compression;
use flate2::write::GzEncoder;
use chrono::{DateTime, Utc};
use clap::ValueEnum;
use std::str::FromStr;

/// only allow explicit values and assign an extension type for each
/// this is used to only allow specific archive types as flags for cli
#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
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
            ArchiveType::Zip => "zip",
        }
    }
}


/// This incorporates some of the archive types along with several other extensions for possible log files
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileType {
    Binary,
    Txt,
    Log,
    Json,
    Csv,
    Xml,
    Gz,
    Tar,
    Zip,
    Unknown,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            FileType::Txt => "txt",
            FileType::Log => "log",
            FileType::Json => "json",
            FileType::Csv => "csv",
            FileType::Xml => "xml",
            FileType::Binary => "bin",
            FileType::Gz => "gz",
            FileType::Tar => "tar",
            FileType::Zip => "zip",
            FileType::Unknown => "unknown",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for FileType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "txt" | "text" => Ok(FileType::Txt),
            "log" => Ok(FileType::Log),
            "json" => Ok(FileType::Json),
            "csv" => Ok(FileType::Csv),
            "xml" => Ok(FileType::Xml),
            "bin" => Ok(FileType::Unknown),
            "gz" => Ok(FileType::Gz),
            "tar" => Ok(FileType::Tar),
            "zip" => Ok(FileType::Zip),
            _ => Ok(FileType::Unknown),
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
pub fn archive_remove_or_truncate_file(file: &str, threshold_days: i64) -> anyhow::Result<i32> {
    let _mtime_diff = get_file_mtime_diff(file)?;
    let _file_extension = get_file_extension(file);

    let check_if_archive_file = match _file_extension.as_str() {
        "gz" => true,
        "tar" => true,
        "zip" => true,
        _ => false,
    };
    
    let check_if_unknown_file = match _file_extension.as_str() {
        "unknown" => true,
        _ => false,
    };

    match _mtime_diff {
        _ if (_mtime_diff > threshold_days) && check_if_archive_file && !check_if_unknown_file => Ok(1), // Remove
        _ if (_mtime_diff <= threshold_days) && !check_if_archive_file && !check_if_unknown_file => Ok(0), // Archive
        _ if (_mtime_diff <= threshold_days) && !check_if_archive_file && !check_if_unknown_file => Ok(2), // Truncate
        _ => Ok(3) // Unchanged
    }
}

/// Create a vector to store all *unfiltered* files in the provided directory
pub fn gather_files_from_directory(dir_path: &str) -> anyhow::Result<Vec<path::PathBuf>> {
    let files: Vec<path::PathBuf> = fs::read_dir(dir_path.to_string())?
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

/// Truncate a provided file
pub fn truncate_file(file_path: &str) {
    let file = fs::File::create(file_path.to_string()).unwrap();
    file.set_len(0).unwrap();
}

/// Create a tarball of a provided file and compress
pub fn tar_gunzip_file(file_path: &str, archive_type: ArchiveType) -> anyhow::Result<()> {
    if archive_type == ArchiveType::TarGunzip {
        let new_file_path = file_path.to_string() + "." +archive_type.as_str();
        let tar_gz_file = fs::File::create(new_file_path.clone())?;

        let encoder = GzEncoder::new(tar_gz_file, Compression::default());
        let mut tar_builder = Builder::new(encoder);
        
        tar_builder.append_path_with_name(new_file_path, file_path.to_string())?;
        tar_builder.finish()?;
        Ok(())
    }
    else { Err(anyhow::anyhow!("Archive Type for 'TarGunzip' did not match expected type"))? }
}

/// Create a non-compressed tarball of a provided file
pub fn tar_file(file_path: &str, archive_type: ArchiveType) -> anyhow::Result<()> {
    if archive_type == ArchiveType::Tar {
        let new_file_path = file_path.to_string() + "." +archive_type.as_str();
        let tar_file = fs::File::create(new_file_path.clone())?;

        let mut tar_builder = Builder::new(tar_file);
        let mut old_file = fs::File::open(file_path.to_string())?;

        tar_builder.append_file(new_file_path, &mut old_file)?;
        tar_builder.finish()?;
        Ok(())
    }
    else { Err(anyhow::anyhow!("Archive Type for 'Tar' did not match expected type"))? }
}

/// Create a zip archive of a provided file
pub fn zip_file(file_path: &str, archive_type: ArchiveType) -> anyhow::Result<()> {
    if archive_type == ArchiveType::Zip {
        let new_file_path = file_path.to_string() + "." +archive_type.as_str();
        let zip_file = fs::File::create(new_file_path.clone())?;

        let mut zip_builder = zip::ZipWriter::new(zip_file);
        let options: zip::write::FileOptions<'_, ()> = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        zip_builder.start_file(file_path.to_string(), options)?;
        zip_builder.finish()?;
        Ok(())
    }
    else { Err(anyhow::anyhow!("Archive Type for 'Zip' did not match expected type"))? }
}

/// Get a file extension type from a provided file path
pub fn get_file_extension(file_path: &str) -> String {
    path::Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| FileType::from_str(ext).unwrap_or(FileType::Unknown))
        .map(|file_type| file_type.to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

/// Remove a provided file via it's path
pub fn remove_file(file_path: &str) {
    fs::remove_file(file_path.to_string()).unwrap();
}

/// Do not worry about testing this function - only renders a file list to stdout
#[cfg(not(tarpaulin_include))]
pub fn dry_run_details(file_list: Vec<path::PathBuf>, threshold_days: i64, archive_type: ArchiveType) {
    for file in file_list {
        let mut _temp_archive_check = "";
        match archive_remove_or_truncate_file(file.to_str().unwrap(), threshold_days).unwrap() {
            0 => println!("File: {} | Status: {} | Action Type: Archiving | File Extension: {}",
                          file.to_str().unwrap(),
                          archive_type.as_str(), 
                          get_file_extension(file.to_str().unwrap()),
            ),
            1 => println!("File: {} | Action Type: Removing | File Extension: {}",
                          file.to_str().unwrap(),
                          get_file_extension(file.to_str().unwrap()),
            ),
            2 => println!("File: {} | Action Type: Truncating | File Extension: {}",
                          file.to_str().unwrap(),
                          get_file_extension(file.to_str().unwrap()),
            ),
            3 => println!("File: {} | Action Type: Unchanged | File Extension: {}",
                          file.to_str().unwrap(),
                          get_file_extension(file.to_str().unwrap()),
            ),
            _ => println!("Base Case Error - Was unable to determine action type"),
        }
    }
}

#[cfg(not(tarpaulin_include))]
pub fn actual_run(file_list: Vec<path::PathBuf>, threshold_days: i64, archive_type: ArchiveType) {
    for file in file_list {
        let mut _temp_archive_check = "";
        match archive_remove_or_truncate_file(file.to_str().unwrap(), threshold_days).unwrap() {
            0 => println!("File: {} | Status: {} | Action Type: Archiving | File Extension: {}",
                          file.to_str().unwrap(),
                          archive_type.as_str(),
                          get_file_extension(file.to_str().unwrap()),
            ),
            1 => println!("File: {} | Action Type: Removing | File Extension: {}",
                          file.to_str().unwrap(),
                          get_file_extension(file.to_str().unwrap()),
            ),
            2 => println!("File: {} | Action Type: Truncating | File Extension: {}",
                          file.to_str().unwrap(),
                          get_file_extension(file.to_str().unwrap()),
            ),
            3 => println!("File: {} | Action Type: Unchanged | File Extension: {}",
                          file.to_str().unwrap(),
                          get_file_extension(file.to_str().unwrap()),
            ),
            _ => println!("Base Case Error - Was unable to determine action type"),
        }
    }
}


/// Fake test function
pub fn test_add(left: u64, right: u64) -> u64 {
    left + right
}