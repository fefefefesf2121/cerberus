use crate::core::CerberusModule;
use dialoguer::{Input, Confirm};
use crate::crypto_utils::{encrypt, decrypt, get_key};
use std::path::Path;

pub enum Mode { Encrypt, Decrypt }
pub struct CryptoModule;

impl CryptoModule {
    pub fn run_with_mode(mode: Mode) -> anyhow::Result<()> {
        let path: String = Input::<String>::new().with_prompt("Path to file:").interact_text()?;
        let p = Path::new(&path);
        if !p.exists() { println!("File not found!"); return Ok(()); }
        
        let data = std::fs::read(&path)?;
        if Confirm::new().with_prompt("Confirm?").interact()? {
            let key = get_key();
            match mode {
                Mode::Encrypt => {
                    let res = encrypt(&data, &key)?;
                    std::fs::write(format!("{}.enc", path), res)?;
                    println!("Encrypted saved as .enc");
                }
                Mode::Decrypt => {
                    let res = decrypt(&data, &key)?;
                    std::fs::write(format!("{}.dec", path), res)?;
                    println!("Decrypted saved as .dec");
                }
            }
        }
        Ok(())
    }
}
impl CerberusModule for CryptoModule {
    fn name(&self) -> &str { "Crypto" }
    fn execute(&self) -> anyhow::Result<()> { Ok(()) }
}