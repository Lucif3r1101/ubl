use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand::RngCore;

/// Derives a 256-bit key from password and salt using Argon2
pub fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let salt = SaltString::encode_b64(salt).unwrap();

    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();

    let hash_output = hash.hash.unwrap(); // FIX: Bind temporary
    let hash_bytes = hash_output.as_bytes();

    let mut key = [0u8; 32];
    key.copy_from_slice(&hash_bytes[..32]);
    key
}

/// Encrypts bytes using AES-GCM. Returns (salt, nonce, ciphertext).
pub fn encrypt(data: &[u8], password: &str) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    let key = derive_key(password, &salt);
    let cipher = Aes256Gcm::new_from_slice(&key).unwrap();

    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce);
    let nonce_obj = Nonce::from_slice(&nonce);

    let ciphertext = cipher.encrypt(nonce_obj, data).unwrap();

    (salt.to_vec(), nonce.to_vec(), ciphertext)
}

/// Decrypts ciphertext using password + salt + nonce
pub fn decrypt(salt: &[u8], nonce: &[u8], ciphertext: &[u8], password: &str) -> Vec<u8> {
    let key = derive_key(password, salt);
    let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
    let nonce_obj = Nonce::from_slice(nonce);

    cipher.decrypt(nonce_obj, ciphertext).unwrap()
}
