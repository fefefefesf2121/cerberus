use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, consts::U12}, KeyInit};

pub struct Cerberus {
    cipher: Aes256Gcm,
}

impl Cerberus {
    pub fn new(key_bytes: &[u8]) -> Self {
        let key = Key::<Aes256Gcm>::from_slice(key_bytes);
        Self {
            cipher: Aes256Gcm::new(key),
        }
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let nonce = Nonce::<U12>::from_slice(b"unique-nonce");
        self.cipher.encrypt(nonce, data)
            .map_err(|e| format!("Encryption failed: {}", e))
    }

    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, String> {
        let nonce = Nonce::<U12>::from_slice(b"unique-nonce");
        self.cipher.decrypt(nonce, encrypted_data)
            .map_err(|e| format!("Decryption failed: {}", e))
    }
}