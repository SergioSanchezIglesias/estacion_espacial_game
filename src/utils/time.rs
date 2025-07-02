use crate::game::GameState;
use chrono::{DateTime, Duration, Utc};

pub struct TimeManager;

impl TimeManager {
    pub fn calculate_elapsed_time(last_update: DateTime<Utc>) -> Duration {
        let now = Utc::now();
        let elapsed = now.signed_duration_since(last_update);
        elapsed.min(Duration::hours(24))
    }

    pub fn apply_time_changes(game_state: &mut GameState, elapsed: Duration) {}
}
