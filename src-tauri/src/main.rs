use json_passwords::PasswordEntry;
use once_cell::sync::OnceCell;
static PASSWORD: OnceCell<String> = OnceCell::new();
use crate::file_checker::create_if_not_exists;
use crate::hash_options::{compare_password, write_hash_to_file};
use crate::json_passwords::{delete_password, get_all_items, password_entry_from_frontend};
use crate::password_config::{generate_password, password_strength};
use crate::AES::{aes_decrypt, aes_encrypt};
mod AES;
mod file_checker;
mod hash_options;
mod json_passwords;
mod password_config;
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// -------------------------------------------------------
// FILE CHECKER OPERATIONS
// -------------------------------------------------------

// check if json file exists wrapper
#[tauri::command]
fn file_check(new_password: String) -> bool {
    // check if json file exists
    create_if_not_exists(&new_password);
    println!("File check complete");

    if compare_password(&new_password) {
        match PASSWORD.set(new_password) {
            Ok(_) => {
                println!("password set");
            }
            Err(err) => {
                println!("error setting password: {}", err);
            }
        }
        return true;
    }
    // store password in memory so it stays in scope
    return false;
}

fn test() {
    if let Some(password) = PASSWORD.get() {
        println!("password is: {}", password);
    } else {
        println!("password not set");
    }
}

// -------------------------------------------------------
// JSON DB OPERATIONS
// -------------------------------------------------------

// write password to json file wrapper
#[tauri::command]
fn write_to_file(entry: PasswordEntry) {
    password_entry_from_frontend(
        &entry.appname,
        &entry.username,
        &entry.password,
        PASSWORD.get().unwrap(),
    );
}

// get all items from json file wrapper
#[tauri::command]
fn get_json_items() -> Vec<String> {
    let encryption_password = PASSWORD.get().unwrap();
    println!("password is: {}", encryption_password);
    get_all_items(encryption_password)
}

// delete a password given an appname wrapper
#[tauri::command]
fn delete_item(appname: &str) {
    println!("Deleting password: {}", appname);
    delete_password(appname, PASSWORD.get().unwrap());
}

// -------------------------------------------------------
// MAIN APP BUILDERS
// -------------------------------------------------------

#[tauri::command]
fn validate_entry(entry: PasswordEntry) -> String {


    if entry.appname.is_empty() {
        return "Appname cannot be empty".to_string();
    }
    if entry.username.is_empty() {
        return "Username cannot be empty".to_string();
    }
    if entry.password.is_empty() {
        return "Password cannot be empty".to_string();
    }


    return "".to_string();
}

//main app builder method
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // file checker
            file_check,
            delete_item,
            // json password interactions
            get_json_items,
            validate_entry,
            password_strength,
            generate_password,
            write_to_file
        ]) // json creation interactions
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    //json test
}
