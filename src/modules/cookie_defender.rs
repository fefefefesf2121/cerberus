use crate::core::CerberusModule;
pub struct CookieDefender;
impl CerberusModule for CookieDefender {
    fn name(&self) -> &str { "Cookie Defender" }
    fn execute(&self) -> anyhow::Result<()> {
        println!("[*] Cookie Defender active.");
        Ok(())
    }
}