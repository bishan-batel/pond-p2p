use std::collections::HashMap;


enum Main {}

#[derive(Debug)]
pub struct App {
    key: String,
    value: String,
    pairs: HashMap<String, String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            key: String::new(),
            value: String::new(),
            pairs: HashMap::new(),
        }
    }
}
