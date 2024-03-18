use std::fs::File;
use std::path::Path;
use std::env;
use std::io::prelude::*;
use crate::hash_options::{write_hash_to_file, compare_password};


pub fn check_if_json_file_exists() -> bool {
    let home_dir = env::var("HOME").expect("Failed to get home directory");
    let path_str = format!("{}/Desktop/PasswordManager/passwords.json", &home_dir);
    let path = Path::new(&path_str);
    let hash_path_str = format!("{}/Desktop/PasswordManager/hash.txt", &home_dir);
    let hash_path: &Path = Path::new(&hash_path_str);
    path.exists() && hash_path.exists()
}

pub fn create_if_not_exists(new_password: &str) {
    if !check_if_json_file_exists() {

        //create 'PasswordManager' directory if it doesn't exist
        let home_dir = env::var("HOME").expect("Failed to get home directory");
        let dir_path = format!("{}/Desktop/PasswordManager", home_dir);
        let path = Path::new(&dir_path);
        if !path.exists() {
            std::fs::create_dir_all(&path).expect("Failed to create directory");
        }
        let home_dir = env::var("HOME").expect("Failed to get home directory");
        let mut file: File = File::create(&format!("{}/Desktop/PasswordManager/passwords.json", home_dir)).unwrap();
        file.write_all(b"hello world!").unwrap();

        let mut hash_file: File = File::create(&format!("{}/Desktop/PasswordManager/hash.txt", home_dir)).unwrap();
        hash_file.write_all(b"hello world!").unwrap();

        //write new password to hash file
        write_hash_to_file(new_password);
    } else {
        //compare new password to hash file
        let result: bool = compare_password(new_password);
    }
}