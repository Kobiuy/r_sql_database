pub struct CommandHistory {
    entries: Vec<String>,
}

impl CommandHistory {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn push(&mut self, serialized: String) {
        self.entries.push(serialized);
    }

    pub fn list(&self) -> &[String] {
        &self.entries
    }
}
