pub struct GameLog {
    pub entries: Vec<String>,
}

impl GameLog {
    pub fn add(&mut self, new_log_entry: String) { self.entries.push(new_log_entry); }
}
