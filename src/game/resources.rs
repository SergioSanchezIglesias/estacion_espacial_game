use serde::{Deserialize, Serialize};

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

    pub fn apply_consumption(&mut self, minutes: i64, crew_size: i32) {
        self.energy = (self.energy - (2 * minutes as i32)).max(0);
        self.oxygen = (self.oxygen - (crew_size * minutes as i32 / 3)).max(0);
        self.food = (self.food - (crew_size * minutes as i32 / 5)).max(0);
    }

    pub fn can_sustain_crew(&self, crew_size: i32) -> bool {
        let min_energy: i32 = 20;
        let min_oxygen: i32 = crew_size;
        let min_food: i32 = crew_size;

        self.energy >= min_energy && self.oxygen >= min_oxygen && self.food >= min_food
    }
}
