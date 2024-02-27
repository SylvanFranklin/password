use crate::file_checker::create_if_not_exists;
use crate::json_passwords::{get_all_items, password_entry_from_frontend};
mod file_checker;
mod json_passwords;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// check if json file exists wrapper
#[tauri::command]
fn file_check() {
    println!("Checking if file exists");
    create_if_not_exists();
}

// write password to json file wrapper
#[tauri::command]
fn write_to_file(app_name: &str, username: &str, password: &str) {
    println!("Writing to file: {} {} {}", app_name, username, password);
    password_entry_from_frontend(app_name, username, password);
}

// get all items from json file wrapper
#[tauri::command]
fn get_json_items() -> Vec<String> {
    get_all_items()
}

//main app builders
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // file checker
            file_check,

            // json password interactions
            get_json_items,
            write_to_file
        ]) // json creation interactions
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    //json test
}
