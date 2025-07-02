use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    pub name: String,
    pub modules: Vec<String>,
    pub max_modules: i32,
}

impl Station {
    pub fn new(name: String) -> Self {
        Station {
            name,
            modules: Vec::new(),
            max_modules: 5,
        }
    }
}
