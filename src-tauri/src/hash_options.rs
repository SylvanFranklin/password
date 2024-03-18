use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{self, BufRead};
use sha256::digest;

pub fn write_hash_to_file(password: &str) {
    // take password and hash it using sha256, replace previous hash in file
    let hash = sha256::digest(password.as_bytes());
    let home_dir = env::var("HOME").unwrap();
    let dir_path = format!("{}/Desktop/PasswordManager", home_dir);
    let path = format!("{}/hash.txt", dir_path);
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .unwrap();
    file.write_all(hash.as_bytes()).unwrap();
}

pub fn compare_password(password: &str) -> bool {
    let home_dir = env::var("HOME").unwrap();
    let dir_path = format!("{}/Desktop/PasswordManager", home_dir);
    let path = format!("{}/hash.txt", dir_path);

    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let password = sha256::digest(password.as_bytes());
        if line == password {
            return true;
        }
    }
    return false;
}
