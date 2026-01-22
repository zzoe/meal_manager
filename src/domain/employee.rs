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
}
