use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};

// Process the information from the folders
pub fn process_input_file(folders: Vec<String>) -> String {
    for folder in folders {
        let file_path = format!("{}/branch_weekly_sales.txt", folder);

        let file = match File::open(&file_path) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("Error opening file {}: {}", file_path, error);
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
                    eprintln!("Error writing to summary file: {}", error);
                    return "ERROR".to_string();
                }
            }                
        }

        let output = format!("{}, {}, {}", branch_code, product_code, total_quantity);

        match write_to_summary_file(&output) {
            Ok(_) => {},
            Err(error) => {
                eprintln!("Error writing to summary file: {}", error);
                return "ERROR".to_string();
            }
            
        }

    }

    "OK".to_string()
}

pub fn write_to_summary_file(output: &str) -> io::Result<()> {
    let summary_file_path = "data/_weekly_summary/summary.txt";

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(summary_file_path)?;

    writeln!(file, "{}", output)?;

    Ok(())
}
