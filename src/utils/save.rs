use crate::game::GameState;
use serde_json;
use std::fs;

#[derive(Debug)]
pub struct SaveManager;

impl SaveManager {
    pub fn new() -> Self {
        SaveManager {}
    }

    pub fn save_game(&self, game: &GameState) -> Result<(), String> {
        let json_string = serde_json::to_string_pretty(&game).map_err(|e| e.to_string())?;

        fs::write("save_game.json", json_string).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn load_game(&self) -> Result<GameState, String> {
        let json_string = fs::read_to_string("save_game.json").map_err(|e| e.to_string())?;

        let game_state: GameState =
            serde_json::from_str(&json_string).map_err(|e| e.to_string())?;

        Ok(game_state)
    }
}
