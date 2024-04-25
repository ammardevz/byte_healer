use std::fs;
use std::io::Write;

use chrono::prelude::*;
use log::LevelFilter;
use simplelog::{CombinedLogger, Config, SimpleLogger, WriteLogger};

pub(crate) struct Logger;

impl Logger {
    pub(crate) fn load_default() {
        let current_exe = std::env::current_exe().unwrap();
        let exe_dir = current_exe.parent().unwrap();

        // Create the "logs" directory if it doesn't exist
        let log_dir = exe_dir.join(crate::config::LOG_DIR);
        if !log_dir.exists() {
            fs::create_dir_all(&log_dir).unwrap();
        }

        // Get the current local date
        let current_date = Local::now();

        // Define the desired date format
        let format = chrono::format::StrftimeItems::new("%Y-%m-%d");

        // Format the date using the specified format
        let formatted_date = current_date.format_with_items(format).to_string();

        // Create the log file path for the current date
        let log_file_path = log_dir.join(format!("log_{}.txt", formatted_date));

        // Check if the log file already exists
        let file_exists = log_file_path.exists();

        // Open the log file in append mode
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(log_file_path)
            .unwrap();

        // Write a separator if the log file is newly created
        if !file_exists {
            let separator = format!("\n\n==================== Log File ({}) ====================\n\n", formatted_date);
            file.write_all(separator.as_bytes()).unwrap()
        }

        // Initialize the logger to write logs to the file
        CombinedLogger::init(vec![
            WriteLogger::new(
                LevelFilter::Debug,
                Config::default(),
                file,
            ),
            SimpleLogger::new(LevelFilter::Off, Config::default()),
        ]).unwrap();

    }
}