use rand::Rng;
use zxcvbn::zxcvbn;
use passwords::PasswordGenerator;

pub fn generate_password(length: u8) -> String {
    let mut rng = rand::thread_rng();
    let password: String = PasswordGenerator::new()
        .length(length)
        .numbers(true)
        .symbols(true)
        .lowercase_letters(true)
        .uppercase_letters(true)
        .generate_one()
        .unwrap();
    password
}

// score between 0 and 4
pub fn password_strength(password: &str) -> u8 {
    let result = zxcvbn(password, &[]).unwrap();
    result.score
}