mod gameplay;
mod parser;

use gameplay::GameState;

#[cfg(feature = "jsbindings")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "pybindings")]
use pyo3::prelude::*;

#[cfg(feature = "jsbindings")]
#[wasm_bindgen]
pub fn getAvailableMoves(game_state_json: &str) -> String {
    let game_state = GameState::new_from_json_str(game_state_json);
    let result = serde_json::to_string(&game_state);
    match result {
        Ok(available_moves) => available_moves,
        Err(err) => err.to_string(),
    }
}

#[cfg(feature = "jsbindings")]
#[wasm_bindgen]
pub fn processMove(game_state_json: &str, game_moves_json: &str) -> String {
    use parser::GameMoves;

    let game_state = GameState::new_from_json_str(game_state_json);
    let new_game_state = game_state.process_moves(GameMoves::new_from_json_str(game_moves_json));
    let result = serde_json::to_string(&new_game_state);
    match result {
        Ok(new_json) => new_json,
        Err(err) => err.to_string(),
    }
}

#[cfg(feature = "pybindings")]
#[pyfunction]
pub fn get_available_moves(game_state_json: &str) -> String {
    let game_state = GameState::new_from_json_str(game_state_json);
    let result = serde_json::to_string(&game_state);
    match result {
        Ok(available_moves) => available_moves,
        Err(err) => err.to_string(),
    }
}

#[cfg(feature = "pybindings")]
#[pyfunction]
pub fn process_move(game_state_json: &str, game_moves_json: &str) -> String {
    use parser::GameMoves;

    let game_state = GameState::new_from_json_str(game_state_json);
    let new_game_state = game_state.process_moves(GameMoves::new_from_json_str(game_moves_json));
    let result = serde_json::to_string(&new_game_state);
    match result {
        Ok(new_json) => new_json,
        Err(err) => err.to_string(),
    }
}

/// A Python module implemented in Rust.
#[cfg(feature = "pybindings")]
#[pymodule]
fn rummy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_available_moves, m)?)?;
    m.add_function(wrap_pyfunction!(process_move, m)?)?;
    Ok(())
}
