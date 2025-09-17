use std::fs;
use std::path;
use tar::Builder;
use flate2::Compression;
use flate2::write::GzEncoder;
use chrono::{DateTime, Utc};
use clap::ValueEnum;

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
pub fn archive_or_remove_file(file: &str, threshold_days: i64, archive_type: ArchiveType) -> anyhow::Result<()> {
    let _mtime_diff = get_file_mtime_diff(file)?;
    if _mtime_diff > threshold_days {
        println!("Archive Type: {}", archive_type.as_str());
        println!("File Path: {}", file);
    }

    Ok(())
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

/// Remove a provided file via it's path
pub fn remove_file(file_path: &str) {
    fs::remove_file(file_path.to_string()).unwrap();
}

/// Do not worry about testing this function - only renders a file list to stdout
pub fn dry_run_details(file_list: Vec<path::PathBuf>) {
    for file in file_list {
        println!("File: {}", file.to_str().unwrap());
    }
}


/// Fake test function
pub fn test_add(left: u64, right: u64) -> u64 {
    left + right
}