use aes_gcm::{Aes256Gcm, Key};
use aes_gcm::aead::{Aead, KeyInit};
use anyhow::{anyhow, Result};

// Фиксированный nonce (в реальных проектах лучше использовать случайный и сохранять его)
const NONCE_BYTES: &[u8; 12] = b"unique nonce";

pub fn encrypt(data: &[u8], key_bytes: &[u8]) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = aes_gcm::Nonce::from_slice(NONCE_BYTES);
    
    cipher.encrypt(nonce, data)
        .map_err(|e| anyhow!("Ошибка шифрования: {}", e))
}

pub fn decrypt(data: &[u8], key_bytes: &[u8]) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = aes_gcm::Nonce::from_slice(NONCE_BYTES);
    
    cipher.decrypt(nonce, data)
        .map_err(|e| anyhow!("Ошибка расшифровки (неверный ключ?): {}", e))
}