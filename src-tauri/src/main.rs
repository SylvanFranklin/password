use crate::json::{check_if_json_file_exists, create_if_not_exists, create_json_file};
use crate::json_passwords::*;

// json interactions
#[macro_use]
mod json;

// json password interactions
#[macro_use]
mod json_passwords;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[tauri::command]
fn greet(name: &str) -> () {
    println!("Hello, {}!", name);
}

#[tauri::command]
fn write_to_file(app_name: &str, username: &str, password: &str) {
    println!("Writing to file: {} {} {}", app_name, username, password);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            check_if_json_file_exists,
            create_if_not_exists
        ]) // json detection interactions
        .invoke_handler(tauri::generate_handler![
            print_all_items,
            get_all_items,
            write_to_file,
        ]) // json creation interactions
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
