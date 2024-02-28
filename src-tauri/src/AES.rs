use openssl::symm::{encrypt, decrypt, Cipher};
use openssl::hash::MessageDigest;
use openssl::pkcs5::pbkdf2_hmac;
use base64;

fn derive_key_from_password(password: &[u8], salt: &[u8]) -> [u8; 32] {
    let mut key = [0; 32]; // AES-256 key size
    pbkdf2_hmac(password, salt, 1000, MessageDigest::sha256(), &mut key).unwrap();
    key
}

pub fn aes_encrypt(password: &[u8], plaintext: &[u8]) -> String {
    let key = derive_key_from_password(password, b"salt1234"); // Use a random salt
    let cipher = Cipher::aes_256_cbc();
    let mut iv = [0; 16]; // Create an array to hold the IV

    openssl::rand::rand_bytes(&mut iv).unwrap(); // Generate a random IV

    let ciphertext = encrypt(cipher, &key, Some(&iv), plaintext).unwrap();

    // Prepend the IV to the ciphertext
    let mut result = Vec::new();
    result.extend_from_slice(&iv[..]); // Pass a reference to the IV slice
    result.extend_from_slice(&ciphertext);

    base64::encode(&result)
}

pub fn aes_decrypt(password: &[u8], ciphertext_with_iv: &str) -> Vec<u8> {
    let key = derive_key_from_password(password, b"salt1234"); // Use the same salt
    let cipher = Cipher::aes_256_cbc();

    // Decode the base64 ciphertext with IV
    let ciphertext_with_iv = base64::decode(ciphertext_with_iv).unwrap();

    // Extract the IV from the beginning of the ciphertext
    let iv = &ciphertext_with_iv[..16];
    let ciphertext = &ciphertext_with_iv[16..];

    decrypt(cipher, &key, Some(iv), ciphertext).unwrap()
}

//fn main() {
//    let password = b"my_secure_password";
//    let plaintext = b"Hello, world!";
//    
//
//    let ciphertext_with_iv = aes_encrypt(password, plaintext);
//    println!("Encrypted (Base64): {:?}", ciphertext_with_iv);
//
//    let decrypted = aes_decrypt(password, &ciphertext_with_iv);
//    println!("Decrypted: {:?}", String::from_utf8_lossy(&decrypted));
//}
