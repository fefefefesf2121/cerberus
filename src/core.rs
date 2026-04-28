pub trait CerberusModule {
    fn name(&self) -> &str;
    fn execute(&self) -> anyhow::Result<()>;
}

pub struct ModuleManager {
    pub modules: Vec<Box<dyn CerberusModule>>,
}

impl ModuleManager {
    pub fn new() -> Self { Self { modules: Vec::new() } }
    pub fn register(&mut self, m: Box<dyn CerberusModule>) { self.modules.push(m); }
}