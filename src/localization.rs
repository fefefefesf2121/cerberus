pub enum Lang { En, Ru }
pub struct Text { pub select_action: String }
impl Text {
    pub fn new(lang: &Lang) -> Self {
        match lang {
            Lang::En => Self { select_action: "Select action:".to_string() },
            Lang::Ru => Self { select_action: "Выберите действие:".to_string() },
        }
    }
}