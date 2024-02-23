use std::fs::File;
use std::io::prelude::*;

#[tauri::command]
pub fn create_json_file() {
    let data = r#"{
        "app": "Google",
        "username": "user1234",
        "password": "password1234"
    }"#;

    if let Err(err) = std::fs::write("passwords.json", data) {
        eprintln!("Failed to write JSON file: {}", err);
    }
}

#[tauri::command]
pub fn check_if_json_file_exists() -> bool {
    let path = std::path::Path::new("passwords.json");
    println!("{}", path.exists());
    path.exists()
}

#[tauri::command]
pub fn create_if_not_exists() {
    if !check_if_json_file_exists() {
        print!("Creating json file passwords.json");
        let mut file: File = File::create("passwords.json").unwrap();
        file.write_all(b"hello world!").unwrap();
    } else {
        print!("File already exists");
    }
}