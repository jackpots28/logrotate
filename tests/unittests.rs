use logrotate::{ArchiveType, archive_file, gather_files_from_directory, get_file_mtime_diff, test_add};
use std::path::PathBuf;
use std::fs::File;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gathering_files_from_directory() {
        let testing_operand = gather_files_from_directory("./tests/test_log_dir")
            .unwrap();

        let right_hand_operand = vec![PathBuf::from("./tests/test_log_dir/test_log_file.log")];

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
    fn test_simple_add() {
        let testing_operand = test_add(1, 2);
        assert_eq!(testing_operand, 3);
    }
}