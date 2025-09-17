use logrotate::{
    ArchiveType,
    archive_file,
    gather_files_from_directory,
    get_file_mtime_diff,
    test_add,
    truncate_file,
    tar_file,
};
use std::path;
use std::fs::File;
use serial_test::serial;

/// Note - test_gathering_files_from_directory and test_tar_file_process are serial tests because they
/// conflict when the new test tar file is created while also checking directory contents

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::empty;
    use super::*;

    #[test]
    #[serial]
    fn test_gathering_files_from_directory() {
        let testing_operand = gather_files_from_directory("./tests/test_log_dir")
            .unwrap();

        let right_hand_operand = vec![path::PathBuf::from("./tests/test_log_dir/test_log_file.log")];

        assert_eq!(testing_operand, right_hand_operand);
    }

    #[test]
    fn test_get_file_mtime() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";

        // Truncate test file to reset mtime to the current day for comparison
        let file = File::create(test_file_path.to_string()).unwrap();
        file.set_len(0).unwrap();

        let testing_operand = get_file_mtime_diff(test_file_path)
            .unwrap();

        assert_eq!(testing_operand, 0);
    }
    
    #[test]
    fn test_archive_file_threshold_check() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";
        let testing_boolean = archive_file(
            test_file_path, 
            0, 
            ArchiveType::Tar,
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
    #[serial]
    fn test_tar_file_process() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";
        let test_new_tar_file= test_file_path.to_string() + ".tar.gz";
        
        tar_file(&test_file_path, ArchiveType::TarGunzip).expect("Error tar-ing file");
        
        assert!(path::Path::new(test_new_tar_file.as_str()).exists());
        
        // Clean up the test tar.gz file that is created
        fs::remove_file(test_new_tar_file.to_string()).unwrap();
    }

    #[test]
    fn test_simple_add() {
        let testing_operand = test_add(1, 2);
        assert_eq!(testing_operand, 3);
    }
}