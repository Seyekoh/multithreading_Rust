use std::fs;
use std::path::Path;
use std::io;
use std::time::Instant;

use project1::process_input_file;

// Main entry point to the program
fn main() -> io::Result<()> {
    let dir_path = "data/_weekly_summary";

    if create_directory_if_not_exists(dir_path) {
        println!("Directory {} created successfully", dir_path);
    } else {
        println!("Directory {} already exists", dir_path);
    }

    let start_time = start_timer();

    let branch_folders = get_folders_for_processing()?;

    let message = process_input_file(branch_folders);
    println!("{}", message);

    stop_timer(start_time);

    println!("Phew! I am done.");

    Ok(())
}

// Function to create a directory if it does not exist
fn create_directory_if_not_exists(dir_path: &str) -> bool {
    if !Path::new(dir_path).exists() {
        fs::create_dir_all(dir_path).expect("Failed to create directory");
        return true;
    }
    false
}

// Starts a timer
fn start_timer() -> Instant {
    Instant::now()
}

// Stops a timer and prints the elapsed time
fn stop_timer(start: Instant) {
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

fn get_folders_for_processing() -> io::Result<Vec<String>> {
    let data_folder = "data";
    let mut branch_folders = Vec::new();

    for entry in fs::read_dir(data_folder)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(folder_name) = path.file_name() {
                if folder_name != "_weekly_summary" {
                    branch_folders.push(path.to_str().unwrap().to_string());
                }
            }
        }
    }

    Ok(branch_folders)
}

