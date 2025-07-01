use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resources {
    pub energy: i32,
    pub oxygen: i32,
    pub food: i32,
    pub materials: i32,
    pub credits: i32,
}

impl Resources {
    pub fn new() -> Self {
        Resources {
            energy: 100,
            oxygen: 100,
            food: 50,
            materials: 25,
            credits: 1000,
        }
    }
}