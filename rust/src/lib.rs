mod gameplay;
mod parser;

use parser::common_parse_game_state;

#[cfg(feature = "jsbindings")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "pybindings")]
use pyo3::prelude::*;

#[cfg(feature = "jsbindings")]
#[wasm_bindgen]
pub fn parseGameState(game_state_json: &str) -> bool {
    common_parse_game_state(game_state_json);
    true
}

#[cfg(feature = "jsbindings")]
#[wasm_bindgen]
pub fn getAvailableMoves(game_state_json: &str) -> String {
    let game_state = common_parse_game_state(game_state_json);
    let result = serde_json::to_string(&gameplay::get_available_moves(game_state));
    match result {
        Ok(available_moves) => available_moves,
        Err(err) => err.to_string(),
    }
}

#[cfg(feature = "pybindings")]
#[pyfunction]
pub fn parse_game_state(game_state_json: &str) -> bool {
    let game_state = common_parse_game_state(game_state_json);
    true
}

#[cfg(feature = "pybindings")]
#[pyfunction]
pub fn get_available_moves(game_state_json: &str) -> String {
    let game_state = common_parse_game_state(game_state_json);
    let result = serde_json::to_string(&gameplay::get_available_moves(game_state));
    match result {
        Ok(available_moves) => available_moves,
        Err(err) => err.to_string(),
    }
}

/// A Python module implemented in Rust.
#[cfg(feature = "pybindings")]
#[pymodule]
fn rummy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_game_state, m)?)?;
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
