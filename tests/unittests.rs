use logrotate::{
    ArchiveType,
    archive_or_remove_file,
    gather_files_from_directory,
    get_file_mtime_diff,
    test_add,
    truncate_file,
    tar_gunzip_file,
    tar_file,
    zip_file,
    remove_file,
};

use std::fs;
use std::path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gathering_files_from_directory() {
        let testing_operand = gather_files_from_directory("./tests/test_log_dir")
            .unwrap();

        // let right_hand_operand = vec![path::PathBuf::from("./tests/test_log_dir/test_log_file.log")];
        // old test - assert_eq!(testing_operand, right_hand_operand);

        assert!(testing_operand.len() >= 1);
    }

    #[test]
    fn test_get_file_mtime() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";

        // Truncate test file to reset mtime to the current day for comparison
        let file = fs::File::create(test_file_path.to_string()).unwrap();
        file.set_len(0).unwrap();

        let testing_operand = get_file_mtime_diff(test_file_path)
            .unwrap();

        assert_eq!(testing_operand, 0);
    }
    
    #[test]
    fn test_tar_archive_file_threshold_check() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";
        let testing_boolean = archive_or_remove_file(
            test_file_path, 
            0, 
            ArchiveType::Tar,
        ).is_ok();
        
        assert_eq!(testing_boolean, true);
    }

    #[test]
    fn test_targunzip_archive_file_threshold_check() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";
        let testing_boolean = archive_or_remove_file(
            test_file_path,
            0,
            ArchiveType::TarGunzip,
        ).is_ok();

        assert_eq!(testing_boolean, true);
    }

    #[test]
    fn test_zip_archive_file_threshold_check() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";
        let testing_boolean = archive_or_remove_file(
            test_file_path,
            0,
            ArchiveType::Zip,
        ).is_ok();

        assert_eq!(testing_boolean, true);
    }
    
    #[test]
    fn test_truncate_file() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";
        truncate_file(test_file_path);
        
        let file_size = fs::metadata(test_file_path.to_string()).unwrap().len();
        assert_eq!(file_size, 0);
    }
    
    #[test]
    fn test_targunzip_file_process() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";
        let test_new_tar_file= test_file_path.to_string() + ".tar.gz";
        
        tar_gunzip_file(&test_file_path, ArchiveType::TarGunzip).expect("Error tar-ing file");
        
        assert!(path::Path::new(test_new_tar_file.as_str()).exists());
        
        // Clean up the test tar.gz file that is created
        fs::remove_file(test_new_tar_file.to_string()).unwrap();
    }

    #[test]
    fn test_tar_file_process() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";
        let test_new_tar_file= test_file_path.to_string() + ".tar";

        tar_file(&test_file_path, ArchiveType::Tar).expect("Error tar-ing file");

        assert!(path::Path::new(test_new_tar_file.as_str()).exists());
        fs::remove_file(test_new_tar_file.to_string()).unwrap();
    }

    #[test]
    fn test_zip_file_process() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";
        let test_new_zip_file= test_file_path.to_string() + ".zip";

        zip_file(&test_file_path, ArchiveType::Zip).expect("Error zipping file");

        assert!(path::Path::new(test_new_zip_file.as_str()).exists());

        // Clean up the test zip file that is created
        fs::remove_file(test_new_zip_file.to_string()).unwrap();
    }

    #[test]
    fn test_remove_file_process() {
        let test_file_path = "./tests/test_log_dir/test_log_file_2.log";
        fs::File::create(test_file_path.to_string()).unwrap();
        assert!(path::Path::new(test_file_path).exists());

        remove_file(test_file_path);
        assert!(!path::Path::new(test_file_path).exists());
    }

    #[test]
    fn test_simple_add() {
        let testing_operand = test_add(1, 2);
        assert_eq!(testing_operand, 3);
    }
}