use std::fs::File;
use std::io::prelude::*;

pub fn check_if_json_file_exists() -> bool {
    let path = std::path::Path::new("passwords.json");
    println!("{}", path.exists());
    path.exists()
}

pub fn create_if_not_exists() {
    if !check_if_json_file_exists() {
        print!("Creating json file passwords.json");
        let mut file: File = File::create("passwords.json").unwrap();
        file.write_all(b"hello world!").unwrap();
    } else {
        print!("File already exists");
    }
}