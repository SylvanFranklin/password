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
    std::path::Path::new("passwords.json").exists()
}

