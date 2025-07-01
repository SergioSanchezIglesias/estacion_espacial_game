use std::fs;
use crate::game::GameState;

#[derive(Debug)]
pub struct SaveManager;

impl SaveManager {
    pub fn new() -> Self {
        SaveManager {}
    }

    pub fn save_game(&self, game: &GameState) -> Result<(), String> {
        Ok(())
    }

    pub fn load_game(&self) -> Result<GameState, String> {
        todo!()
    }
}