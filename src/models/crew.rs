use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrewMember {
    pub name: String,
    pub role: String,
    pub health: i32,
    pub morale: i32,
}

impl CrewMember {
    pub fn new(name: String, role: String) -> Self {
        CrewMember {
            name,
            role,
            health: 100,
            morale: 100,
        }
    }
}
