use logrotate::{
    ArchiveType,
    archive_remove_truncate_file_bucketing,
    gather_files_from_directory,
    get_file_mtime_diff,
    test_add,
    truncate_file,
    tar_gunzip_file,
    tar_file,
    zip_file,
    remove_file,
    get_date,
};

use std::fs;
use std::path;


/// Helper function to create test files
pub fn creat_test_file(extension: &str) -> String {
    let new_file_path = "./tests/test_log_dir/test_".to_string() + extension + "_file." + extension;
    fs::File::create(new_file_path.clone()).ok();
    new_file_path
}


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


    /// These tests revolve around the specific supported files times for rotation
    /// Since the test creates the file, the mtime is always the current date,
    /// so a threshold has to be negative to induce "change"
    #[test]
    fn test_txt_file_archive() {
        let txt_test_file = creat_test_file("txt");
        let result = archive_remove_truncate_file_bucketing(
            txt_test_file.as_str(),
            -1).unwrap();

        assert_eq!(result, 3)
    }

    #[test]
    fn test_text_file_archive() {
        let text_test_file = creat_test_file("text");
        let result = archive_remove_truncate_file_bucketing(
            text_test_file.as_str(),
            -1).unwrap();

        assert_eq!(result, 3)
    }

    #[test]
    fn test_xml_file_archive() {
        let xml_test_file = creat_test_file("xml");
        let result = archive_remove_truncate_file_bucketing(
            xml_test_file.as_str(),
            -1).unwrap();

        assert_eq!(result, 3)
    }

    #[test]
    fn test_log_file_archive() {
        let log_test_file = creat_test_file("log");
        let result = archive_remove_truncate_file_bucketing(
            log_test_file.as_str(),
            -1).unwrap();

        assert_eq!(result, 3)
    }

    #[test]
    fn test_logs_file_archive() {
        let logs_test_file = creat_test_file("logs");
        let result = archive_remove_truncate_file_bucketing(
            logs_test_file.as_str(),
            -1).unwrap();

        assert_eq!(result, 3)
    }

    #[test]
    fn test_clf_file_archive() {
        let clf_test_file = creat_test_file("clf");
        let result = archive_remove_truncate_file_bucketing(
            clf_test_file.as_str(),
            -1).unwrap();

        assert_eq!(result, 3)
    }

    #[test]
    fn test_cef_file_archive() {
        let cef_test_file = creat_test_file("cef");
        let result = archive_remove_truncate_file_bucketing(
            cef_test_file.as_str(),
            -1).unwrap();

        assert_eq!(result, 3)
    }

    #[test]
    fn test_syslog_file_archive() {
        let syslog_test_file = creat_test_file("syslog");
        let result = archive_remove_truncate_file_bucketing(
            syslog_test_file.as_str(),
            -1).unwrap();

        assert_eq!(result, 3)
    }

    #[test]
    fn test_json_file_archive() {
        let json_test_file = creat_test_file("json");
        let result = archive_remove_truncate_file_bucketing(
            json_test_file.as_str(),
            -1).unwrap();

        assert_eq!(result, 3)
    }

    #[test]
    fn test_csv_file_archive() {
        let csv_test_file = creat_test_file("csv");
        let result = archive_remove_truncate_file_bucketing(
            csv_test_file.as_str(),
            -1).unwrap();

        assert_eq!(result, 3)
    }

    #[test]
    fn test_get_file_mtime_diff() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";

        // Truncate test file to reset mtime to the current day for comparison
        let file = fs::File::create(test_file_path.to_string()).unwrap();
        file.set_len(0).unwrap();

        let diff_testing_operand = get_file_mtime_diff(test_file_path).unwrap();
        let ok_testing_operand = get_file_mtime_diff(test_file_path);

        assert_eq!(diff_testing_operand, 0);
        assert_eq!(ok_testing_operand.ok(), Some(0));
    }
    
    #[test]
    fn test_archive_or_remove_file_threshold_check() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";
        let testing_greater_than_threshold = archive_remove_truncate_file_bucketing(
            test_file_path, 
            1,
        ).unwrap();

        let testing_less_than_threshold = archive_remove_truncate_file_bucketing(
            test_file_path, 
            -1,
        ).unwrap();
        
        assert_eq!(testing_greater_than_threshold, 0);
        assert_eq!(testing_less_than_threshold, 3);
    }
    
    #[test]
    fn test_tar_archive_file_threshold_check() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";
        let testing_boolean = archive_remove_truncate_file_bucketing(
            test_file_path, 
            0,
        ).is_ok();
        
        assert!(testing_boolean);
    }

    #[test]
    fn test_targunzip_archive_file_threshold_check() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";
        let testing_boolean = archive_remove_truncate_file_bucketing(
            test_file_path,
            0,
        ).is_ok();

        assert!(testing_boolean);
    }

    #[test]
    fn test_zip_archive_file_threshold_check() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";
        let testing_boolean = archive_remove_truncate_file_bucketing(
            test_file_path,
            0,
        ).is_ok();

        assert!(testing_boolean);
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
        let test_new_tar_file= test_file_path.to_string() + "_" + &get_date() + ".tar.gz";
        
        tar_gunzip_file(&test_file_path, ArchiveType::TarGunzip).expect("Error tar-ing file");
        
        assert!(path::Path::new(test_new_tar_file.as_str()).exists());
        
        // Clean up the test tar.gz file that is created
        fs::remove_file(test_new_tar_file.to_string()).unwrap();
    }

    #[test]
    fn test_tar_file_process() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";
        let test_new_tar_file= test_file_path.to_string() + "_" + &get_date() + ".tar";

        tar_file(&test_file_path, ArchiveType::Tar).expect("Error tar-ing file");

        assert!(path::Path::new(test_new_tar_file.as_str()).exists());
        fs::remove_file(test_new_tar_file.to_string()).unwrap();
    }

    #[test]
    fn test_zip_file_process() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";
        let test_new_zip_file= test_file_path.to_string() + "_" + &get_date() + ".zip";

        zip_file(&test_file_path, ArchiveType::Zip).expect("Error zipping file");

        assert!(path::Path::new(test_new_zip_file.as_str()).exists());

        // Clean up the test zip file that is created
        fs::remove_file(test_new_zip_file.to_string()).unwrap();
    }

    #[test]
    fn test_wrong_archive_types() {
        let test_file_path = "./tests/test_log_dir/test_log_file.log";

        let tar_result = tar_file(test_file_path, ArchiveType::TarGunzip);
        let targunzip_result = tar_gunzip_file(test_file_path, ArchiveType::Zip);
        let zip_result = zip_file(test_file_path, ArchiveType::Tar);

        assert!(tar_result.is_err());
        assert!(targunzip_result.is_err());
        assert!(zip_result.is_err());
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