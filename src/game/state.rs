use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::game::Resources;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub resources: Resources,
    pub station_name: String,
    pub current_day: i32,
    pub last_update: DateTime<Utc>,
}

impl GameState {
    pub fn new(name: String) -> Self {
        GameState {
            resources: Resources::new(),
            station_name: name.to_string(),
            current_day: 1,
            last_update: Utc::now(),
        }
    }
}