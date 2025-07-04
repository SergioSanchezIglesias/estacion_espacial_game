mod game;
mod models;
mod ui;
mod utils;

use game::GameState;
use models::{CrewMember, Station};
use ui::Display;
use utils::{SaveManager, TimeManager};

fn main() {
    let display = Display::new();
    let save_manager = SaveManager::new();

    display.show_welcome();

    let mut game_state = match save_manager.load_game() {
        Ok(state) => {
            println!("Partida cargada correctamente");
            state
        }
        Err(_) => {
            println!("No se encontró partida guardada. Iniciando nueva partida...");
            GameState::new("Estación Alpha".to_string())
        }
    };

    // Mostrar información del juego
    println!("\n=== ESTADO DE LA ESTACIÓN ===");
    println!("Estación: {}", game_state.station_name);
    println!("Día: {}", game_state.current_day);
    println!(
        "Recursos: Energía:{}, Oxígeno:{}, Comida:{}, Materiales:{}, Créditos:{}",
        game_state.resources.energy,
        game_state.resources.oxygen,
        game_state.resources.food,
        game_state.resources.materials,
        game_state.resources.credits
    );

    let elapsed = TimeManager::calculate_elapsed_time(game_state.last_update);
    println!("Tiempo transcurrido: {} minutos", elapsed.num_minutes());

    TimeManager::apply_time_changes(&mut game_state, elapsed);

    match save_manager.save_game(&game_state) {
        Ok(_) => println!("\nPartida guardada correctamente"),
        Err(e) => println!("\nError al guardar la partida: {}", e),
    }

    println!("¡Hasta la próxima, comandante!");
}
