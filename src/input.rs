use std::fs::read_to_string;
pub fn read_file(filepath: &str) -> String {
    return read_to_string(filepath).expect("Unable to read file");
}
