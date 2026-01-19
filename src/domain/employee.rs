use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub name: String,
    pub aliases: Vec<String>,
}

impl Employee {
    pub fn new(name: String, aliases: Vec<String>) -> Self {
        Self { name, aliases }
    }

    pub fn has_alias(&self, alias: &str) -> bool {
        self.name == alias || self.aliases.iter().any(|a| a == alias)
    }
}
