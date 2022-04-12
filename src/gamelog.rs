pub struct GameLog {
    entries: Vec<String>,
}

impl Default for GameLog {
    fn default() -> Self {
        Self {
            entries: vec!["Welcome to Rusty Roguelike".to_string()],
        }
    }
}

impl GameLog {
    pub fn add(&mut self, new_log_entry: String) { self.entries.push(new_log_entry); }

    pub fn get_log(&self) -> Vec<String> { self.entries.clone() }
}
