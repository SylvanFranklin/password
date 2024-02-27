use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, SeekFrom};
use std::env;

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

#[tauri::command]
pub fn get_all_passwords() -> io::Result<Vec<PasswordEntry>> {
    read_passwords()
}

pub fn add_password(entry: PasswordEntry) -> io::Result<()> {
    let mut passwords = read_passwords().unwrap_or_else(|_| vec![]);
    passwords.push(entry);
    write_passwords(&passwords)?;
    Ok(())
}

#[tauri::command]
pub fn delete_password(appname: &str) -> io::Result<()> {
    let mut passwords = read_passwords().unwrap_or_else(|_| vec![]);
    passwords.retain(|entry| entry.appname != appname);
    write_passwords(&passwords)?;
    Ok(())
}

#[tauri::command]
pub fn get_data(entry: PasswordEntry) -> (String, String, String) {
    (entry.appname, entry.username, entry.password)
}

#[tauri::command]
pub fn print_all_items() {
    let passwords = get_all_passwords().unwrap_or_else(|_| vec![]);
    for entry in passwords {
        let (appname, username, password) = get_data(entry);
        println!(
            "Appname: {}\nUsername: {}\nPassword: {}\n",
            appname, username, password
        );
    }
}

pub fn get_all_items() -> Vec<String> {
    let mut items = vec![];
    let passwords = get_all_passwords().unwrap_or_else(|_| vec![]);
    for entry in passwords {
        let (appname, username, password) = get_data(entry);
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
pub fn password_entry_from_frontend(appname: &str, username: &str, password: &str) -> () {
    let new_entry = PasswordEntry {
        appname: appname.to_string(),
        username: username.to_string(),
        password: password.to_string(),
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
