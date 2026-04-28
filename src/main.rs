mod core;
mod modules;
mod crypto_utils;

use modules::crypto_module::{CryptoModule, Mode};
use modules::maintenance::MaintenanceModule;
use dialoguer::Select;

fn main() -> anyhow::Result<()> {
    println!("Cerberus System v1.2.6");
    loop {
        let items = vec!["Encrypt", "Decrypt", "Maintenance", "Exit"];
        let sel = Select::new().items(&items).interact()?;
        match sel {
            0 => CryptoModule::run_with_mode(Mode::Encrypt)?,
            1 => CryptoModule::run_with_mode(Mode::Decrypt)?,
            2 => MaintenanceModule::cleanup()?,
            _ => break,
        }
    }
    Ok(())
}