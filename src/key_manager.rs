use keyring::Entry;
use rand::{RngCore, thread_rng};

pub fn get_or_create_key() -> Vec<u8> {
    // Создаем запись в "Диспетчере учетных данных"
    let entry = Entry::new("cerberus_app", "master_key").expect("Ошибка доступа к Keyring");

    match entry.get_password() {
        Ok(hex_key) => hex::decode(hex_key).expect("Ключ в системе поврежден"),
        Err(_) => {
            // Если ключа нет, генерируем новый
            let mut key = vec![0u8; 32];
            thread_rng().fill_bytes(&mut key);
            
            // Сохраняем в систему (в hex-формате)
            let hex_key = hex::encode(&key);
            entry.set_password(&hex_key).expect("Не удалось сохранить ключ в системе");
            key
        }
    }
}