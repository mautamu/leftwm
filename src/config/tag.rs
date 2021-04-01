use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Tag {
    pub name: String,
    pub layout: Option<String>,
}

impl Tag {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn new(s: String) -> Self {
        Self {
            name: s,
            layout: None,
        }
    }
}
