use std::fs::{self, File};
#[allow(dead_code)]
pub fn create_directory(path: &str) {
    fs::create_dir_all(path).expect("Failed to create directory");
}
#[allow(dead_code)]
pub fn create_file(path: &str) {
    File::create(path).expect("Failed to create file");
}