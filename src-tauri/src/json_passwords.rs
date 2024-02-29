use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, SeekFrom};
use std::env;
use tauri::dialog::{Dialog, DialogBuilder}; //file popup window

// encryption
use crate::AES::{aes_decrypt, aes_encrypt};

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub appname: String,
    pub username: String,
    pub password: String,
}

impl PasswordEntry {
    pub fn new(appname: String, username: String, password: String) -> Self {
        PasswordEntry {
            appname,
            username,
            password,
        }
    }
}

#[tauri::command]
pub fn read_passwords() -> io::Result<Vec<PasswordEntry>> {
    let home_dir = env::var("HOME").expect("Failed to get home directory");
    let file = File::open(&format!("{}/Desktop/PasswordManager/passwords.json", home_dir))?;
    let reader = io::BufReader::new(file);
    let passwords: Vec<PasswordEntry> = serde_json::from_reader(reader)?;
    Ok(passwords)
}

pub fn write_passwords(passwords: &[PasswordEntry]) -> io::Result<()> {
    let home_dir = env::var("HOME").expect("Failed to get home directory");
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&format!("{}/Desktop/PasswordManager/passwords.json", home_dir))?;

    serde_json::to_writer_pretty(file, &passwords)?;
    Ok(())
}

pub fn get_all_passwords() -> io::Result<Vec<PasswordEntry>> {
    read_passwords()
}

pub fn add_password(entry: PasswordEntry) -> io::Result<()> {
    let mut passwords = read_passwords().unwrap_or_else(|_| vec![]);
    passwords.push(entry);
    write_passwords(&passwords)?;
    Ok(())
}

pub fn delete_password(appname: &str, encryption_password: &str) -> io::Result<()> {
    let mut passwords = read_passwords().unwrap_or_else(|_| vec![]);
    passwords.retain(|entry| {
        let decrypted_appname = aes_decrypt(encryption_password.as_bytes(), &entry.appname);
        decrypted_appname != appname
    });
    write_passwords(&passwords)?;
    Ok(())
}

#[tauri::command]
pub fn get_data(entry: PasswordEntry) -> (String, String, String) {
    (entry.appname, entry.username, entry.password)
}

pub fn get_all_items(encryption_password: &str) -> Vec<String> {
    let mut items = vec![];
    println!("password backend is: {}", encryption_password);
    let passwords = get_all_passwords().unwrap_or_else(|_| vec![]);
    for entry in passwords {
        let (appname, username, password) = get_data(entry);
        let appname: String= aes_decrypt(encryption_password.as_bytes(), &appname);
        let username: String= aes_decrypt(encryption_password.as_bytes(), &username);
        let password: String= aes_decrypt(encryption_password.as_bytes(), &password);
        let json_data = json!({
            "appname": appname,
            "username": username,
            "password": password
        });

        items.push(json_data.to_string());
    }
    items
}

#[tauri::command]
pub fn password_entry_from_frontend(appname: &str, username: &str, password: &str, encryption_password: &str) {
    //encrypt all items
    let appname_encrypted = aes_encrypt(encryption_password.as_bytes(), appname.as_bytes());
    let username_encrypted = aes_encrypt(encryption_password.as_bytes(), username.as_bytes());
    let password_encrypted = aes_encrypt(encryption_password.as_bytes(), password.as_bytes());
    
    let new_entry = PasswordEntry {
        appname: appname_encrypted,
        username: username_encrypted,
        password: password_encrypted,
    };
    add_password(new_entry).unwrap();
}

// Example usage
//let passwords = get_all_passwords()?;
//println!("Current passwords: {:?}", passwords);

// Add a new password entry
//let new_entry = PasswordEntry::new("Google".to_string(), "userme".to_string(), "tdasdd".to_string());
//add_password(new_entry)?;

// Delete an existing password entry
//delete_password("ExampleApp")?;

// Read passwords again after modifications
//let passwords = get_all_passwords()?;
//format passwords vector by new line
//for entry in passwords {
//    let (appname, username, password) = get_data(entry);
//    println!("Appname: {}\nUsername: {}\nPassword: {}\n", appname, username, password);
//}


// -------------------------------------------------------
// Dialog Popups
// -------------------------------------------------------

//returns the path of the file selected, and writes the path to a file
use std::fs::File;
use std::io::Write;
use std::env;
use std::fs::File;
use std::io::{Read, Write};

pub async fn file_popup() -> String {
    let dialog = DialogBuilder::new()
        .title("Open File")
        .filters(vec![("JSON Files", vec!["json"])])
        .show_open_single()
        .build()
        .unwrap();

    let result = dialog.run().await.unwrap();
    let path = result.path().to_str().unwrap().to_string();

    // Write the path to an environment variable
    env::set_var("CACHE_FILE_PATH", &path);

    // Write the path to a file, and that file should be in that path
    let mut file = File::create(path.clone() + "/cache.txt").unwrap();
    file.write_all(path.as_bytes()).unwrap();

    path
}

pub fn get_cached_path() -> Option<String> {
    // Check if the environment variable is set
    if let Ok(path) = env::var("CACHE_FILE_PATH") {
        return Some(path);
    }

    // If the environment variable is not set, try to read from the file
    let mut file = File::open("cache.txt").ok()?;
    let mut path = String::new();
    file.read_to_string(&mut path).ok()?;
    Some(path)
}