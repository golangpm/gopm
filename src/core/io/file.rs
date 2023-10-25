use std::fs::{self, File};

pub fn create_directory(path: &str) {
    fs::create_dir_all(path).expect("Failed to create directory");
}

pub fn create_file(path: &str) {
    File::create(path).expect("Failed to create file");
}