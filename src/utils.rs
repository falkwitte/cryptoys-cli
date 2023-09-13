use std::fs::{read_to_string, write};
use std::path::PathBuf;

pub fn extract_file_content(file: String) -> String {
    let file_path = PathBuf::from(file);
    read_to_string(file_path).unwrap()
}

pub fn write_to_output_file(content: String) {
    write(PathBuf::from("output.txt"), content).unwrap();
}
