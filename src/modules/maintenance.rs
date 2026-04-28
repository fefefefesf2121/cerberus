use crate::core::CerberusModule;
use dialoguer::Confirm;
use std::fs;

pub struct MaintenanceModule;
impl MaintenanceModule {
    pub fn cleanup() -> anyhow::Result<()> {
        if Confirm::new().with_prompt("Delete all keys and temp files?").interact()? {
            for entry in fs::read_dir(".")? {
                let path = entry?.path();
                let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
                if ext == "enc" || ext == "dec" || path.file_name().unwrap() == "key.bin" {
                    let _ = fs::remove_file(path);
                }
            }
            println!("Cleaned.");
        }
        Ok(())
    }
}
impl CerberusModule for MaintenanceModule {
    fn name(&self) -> &str { "Maintenance" }
    fn execute(&self) -> anyhow::Result<()> { Self::cleanup() }
}