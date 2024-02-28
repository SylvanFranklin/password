use rand::Rng;
use zxcvbn::zxcvbn;

pub fn generate_password(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789!@#$%&*";
    let mut rng = rand::thread_rng();

    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    password
}

// score between 0 and 4
pub fn password_strength(password: &str) -> u8 {
    let result = zxcvbn(password, &[]).unwrap();
    result.score
}