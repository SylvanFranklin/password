use passwords::PasswordGenerator;
use zxcvbn::zxcvbn;

#[tauri::command]
pub fn generate_password(length: u8) -> String {
    let password: String = PasswordGenerator::new()
        .length(length.into())
        .numbers(true)
        .symbols(true)
        .lowercase_letters(true)
        .uppercase_letters(true)
        .generate_one()
        .unwrap();
    password
}

// score between 0 and 4
#[tauri::command]
pub fn password_strength(password: &str) -> u8 {
    let result = match zxcvbn(password, &[]) {
        Ok(result) => result,
        Err(_) => return 0
    };
    result.score()
}


// WIP Implementation

// trait PasswordGenerator {
//     fn generate_password(&self) -> String;
// }

// struct SimplePasswordGenerator {
//     length: u8,
//     numbers: bool,
//     symbols: bool,
//     lowercase_letters: bool,
//     uppercase_letters: bool,
// }

// impl PasswordGenerator for SimplePasswordGenerator {
//     fn generate_password(&self) -> String {
//         let mut rng = rand::thread_rng();
//         let mut password = String::new();
//         let mut char_set = String::new();
//         if self.numbers {
//             char_set.push_str("0123456789");
//         }
//         if self.symbols {
//             char_set.push_str("!@#$%^&*()-_=+[]{}|;:,.<>/?");
//         }
//         if self.lowercase_letters {
//             char_set.push_str("abcdefghijklmnopqrstuvwxyz");
//         }
//         if self.uppercase_letters {
//             char_set.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
//         }
//         for _ in 0..self.length {
//             let random_index = rng.gen_range(0..char_set.len());
//             password.push(char_set.chars().nth(random_index).unwrap());
//         }
//         password
//     }
// }

// #[tauri::command]
// pub fn generate_simple_password(length: u8, numbers: bool, symbols: bool, lowercase_letters: bool, uppercase_letters: bool) -> String {
//     let generator = SimplePasswordGenerator {
//         length,
//         numbers,
//         symbols,
//         lowercase_letters,
//         uppercase_letters,
//     };
//     generator.generate_password()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_generate_simple_password() {
//         let password = generate_simple_password(10, true, true, true, true);
//         assert_eq!(password.len(), 10);
//     }

//     #[test]
//     fn test_password_strength() {
//         let password = "password";
//         let score = password_strength(password);
//         assert_eq!(score, 0);
//     }
// }

// // Path: src-tauri/src/password_config.rs
>>>>>>> theirs
