use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

// Define constants for the directory and file names
const OUTPUT_DIR: &str = "output";
const FILE_NAME: &str = "input.bin";

fn main() -> io::Result<()> {
    let number: u64 = 45;

    // Ensure the output directory exists
    let output_dir = Path::new(OUTPUT_DIR);
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?; // Create the directory and any necessary parent directories
    }

    // Create the file and write the number in little-endian format
    let file_path = output_dir.join(FILE_NAME);
    let mut file = File::create(&file_path)?;
    file.write_all(&number.to_le_bytes())?; // Convert the number to bytes and write to the file

    Ok(())
}
