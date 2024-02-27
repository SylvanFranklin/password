use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

pub fn check_if_json_file_exists() -> bool {
    let path = Path::new("passwords.json");
    let hash_path: &Path = Path::new("hash.txt");
    println!("JSON DB: {}, HASH DOC: {}", path.exists(), hash_path.exists()); // Added closing parenthesis
    (path.exists() && hash_path.exists())
}

pub fn create_if_not_exists() {
    if !check_if_json_file_exists() {
        print!("Creating json file passwords.json");
        let mut file: File = File::create("passwords.json").unwrap();
        file.write_all(b"hello world!").unwrap();

        print!("Creating hash file hash.txt");
        let mut hash_file: File = File::create("hash.txt").unwrap();
        hash_file.write_all(b"hello world!").unwrap();
    } else {
        print!("File already exists");
    }
}