use std::io::{self, BufRead};
use std::fs::{File, OpenOptions};
use std::sync::mpsc::Sender;
use log::{error, info};
use simplelog::*;

// Initialize logging
pub fn initialize_logger() {
    // Create log.txt file
    let log_file = OpenOptions::new()
    .create(true)
    .write(true)
    .append(true)
    .open("log.txt")
    .unwrap();
   
   // Create Logger
   let _ = WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        log_file,
   ).unwrap();
}

// Process the information from the folders
pub fn process_input_file(folders: Vec<String>, tx: Sender<String>) -> String {
    for folder in folders {
        let file_path = format!("{}/branch_weekly_sales.txt", folder);

        let file = match File::open(&file_path) {
            Ok(file) => file,
            Err(error) => {
                error!("Error opening file {}: {}", file_path, error);
                return "ERROR".to_string();
            }
        };

        let reader = io::BufReader::new(file);
        let mut total_quantity = 0;
        let mut branch_code = String::new();
        let mut product_code = String::new();

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

                    if parts.len() == 4 {
                        branch_code = parts[0].to_string();
                        product_code = parts[1].to_string();
                        let quantity: i32 = parts[2].parse().unwrap_or(0);
                        total_quantity += quantity;
                    }
                }
                Err(error) => {
                    error!("Error writing to summary file: {}", error);
                    return "ERROR".to_string();
                }
            }                
        }

        let output = format!("{}, {}, {}", branch_code, product_code, total_quantity);
        if let Err(error) = tx.send(output) {
            error!("Error sending output: {}", error);
            return "Error".to_string();
        }

        info!("Proccessed folder: {}", folder);

    }

    "OK".to_string()
}
