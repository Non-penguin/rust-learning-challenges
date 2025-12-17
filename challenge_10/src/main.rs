use std::fs::File;
use std::io;

fn open_file_safety() -> Result<File, io::Error> {
    let f = File::open("data.txt")?;
    Ok(f)
}

fn main() {
    open_file_safety().expect("Failed to open file");
    let _file = open_file_safety().expect("Fatal error opening the file");

    println!("File opened successfully.")
}