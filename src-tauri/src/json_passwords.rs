use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, SeekFrom};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct PasswordEntry {
    appname: String,
    username: String,
    password: String,
}

impl PasswordEntry {
    fn new(appname: String, username: String, password: String) -> Self {
        PasswordEntry { appname, username, password }
    }
}

#[tauri::command]
fn read_passwords() -> io::Result<Vec<PasswordEntry>> {
    let file = File::open("passwords.json")?;
    let reader = io::BufReader::new(file);
    let passwords: Vec<PasswordEntry> = serde_json::from_reader(reader)?;
    Ok(passwords)
}

#[tauri::command]
fn write_passwords(passwords: &[PasswordEntry]) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("passwords.json")?;

    serde_json::to_writer_pretty(file, &passwords)?;
    Ok(())
}

#[tauri::command]
fn get_all_passwords() -> io::Result<Vec<PasswordEntry>> {
    read_passwords()
}

#[tauri::command]
fn add_password(entry: PasswordEntry) -> io::Result<()> {
    let mut passwords = read_passwords().unwrap_or_else(|_| vec![]);
    passwords.push(entry);
    write_passwords(&passwords)?;
    Ok(())
}

#[tauri::command]
fn delete_password(appname: &str) -> io::Result<()> {
    let mut passwords = read_passwords().unwrap_or_else(|_| vec![]);
    passwords.retain(|entry| entry.appname != appname);
    write_passwords(&passwords)?;
    Ok(())
}

#[tauri::command]
fn get_data(entry: PasswordEntry) -> (String, String, String) {
    (entry.appname, entry.username, entry.password)
}

#[tauri::command]
pub fn print_all_items() {
    let passwords = get_all_passwords().unwrap_or_else(|_| vec![]);
    for entry in passwords {
        let (appname, username, password) = get_data(entry);
        println!("Appname: {}\nUsername: {}\nPassword: {}\n", appname, username, password);
    }
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
