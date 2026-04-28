use aes_gcm::{Aes256Gcm, Key, KeyInit, aead::{Aead, AeadCore, OsRng}};
use std::fs;
use rand::RngCore;

pub type Result<T> = std::result::Result<T, anyhow::Error>;

pub fn get_key() -> Vec<u8> {
    let path = "key.bin";
    if let Ok(key) = fs::read(path) {
        if key.len() == 32 { return key; }
    }
    let mut key = vec![0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    let _ = fs::write(path, &key);
    key
}

pub fn encrypt(data: &[u8], key_bytes: &[u8]) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let mut ciphertext = cipher.encrypt(&nonce, data).map_err(|e| anyhow::anyhow!("{}", e))?;
    let mut result = nonce.to_vec();
    result.append(&mut ciphertext);
    Ok(result)
}

pub fn decrypt(data: &[u8], key_bytes: &[u8]) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    if data.len() < 12 { return Err(anyhow::anyhow!("Invalid data")); }
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce = aes_gcm::Nonce::from_slice(nonce_bytes);
    let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| anyhow::anyhow!("{}", e))?;
    Ok(plaintext)
}