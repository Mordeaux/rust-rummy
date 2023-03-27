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
    let result = serde_json::to_string(&game_state.get_available_moves());
    match result {
        Ok(available_moves) => available_moves,
        Err(err) => err.to_string(),
    }
}

#[cfg(feature = "pybindings")]
#[pyfunction]
pub fn get_available_moves(game_state_json: &str) -> String {
    let game_state = GameState::new_from_json_str(game_state_json);
    let result = serde_json::to_string(&game_state.get_available_moves());
    match result {
        Ok(available_moves) => available_moves,
        Err(err) => err.to_string(),
    }
}

/// A Python module implemented in Rust.
#[cfg(feature = "pybindings")]
#[pymodule]
fn rummy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_available_moves, m)?)?;
    Ok(())
}

#[cfg(feature = "jsbindings")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
