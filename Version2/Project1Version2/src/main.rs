use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::{self, Write};
use std::time::Instant;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;

static NTHREADS: i32 = 4;

use project1::{process_input_file, initialize_logger};

// Main entry point to the program
fn main() -> io::Result<()> {
    initialize_logger();

    let dir_path = "data/_weekly_summary";

    if create_directory_if_not_exists(dir_path) {
        println!("Directory {} created successfully", dir_path);
    } else {
        println!("Directory {} already exists", dir_path);
    }

    // Creates channels for message sending
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut children: Vec<JoinHandle<()>> = Vec::new();

    // Creates groups of folders for threads
    let branch_folders = get_folders_for_processing()?;
    let thread_groups: Vec<Vec<String>> = branch_folders.chunks(10).map(|chunk| chunk.to_vec()).collect();

    // Ensure num threads is limited by num groups
    let num_threads = std::cmp::min(NTHREADS, thread_groups.len().try_into().unwrap());
    
    let start_time = start_timer();
    
    // Creates threads and sends them to process input file
    for id in 0..num_threads {
        let group = thread_groups[id as usize].clone();
        let tx_clone = tx.clone();

        let child = thread::spawn(move || {
            process_input_file(group, tx_clone);
        });

        children.push(child);
    }

    drop(tx);

    // Waits for threads to finish working
    for child in children {
        child.join().expect("child panicked!");
    }
    
    // Collect output from file processing
    let mut output = String::new();
    for received in rx {
        println!("Received: {}", received);
        output.push_str(&received);
        output.push('\n');
    }

   let _ = write_to_summary_file(&output);

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

pub fn write_to_summary_file(output: &str) -> io::Result<()> {
    let summary_file_path = "data/_weekly_summary/summary.txt";

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(summary_file_path)?;

    writeln!(file, "{}", output)?;

    Ok(())
}