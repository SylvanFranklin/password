
fn create_json_file() -> Result<(), Box<dyn std::error::Error>> {
    let data = r#"{
        "app": "Google",
        "username": "user1234",
        "password": "password1234"
    }"#;

    std::fs::write("passwords.json", data)?;

    Ok(())
}