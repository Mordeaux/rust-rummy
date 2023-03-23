mod parser;

use parser::common_parse_game_state;

#[cfg(feature = "jsbindings")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "pybindings")]
use pyo3::prelude::*;

#[cfg(feature = "jsbindings")]
#[wasm_bindgen]
pub fn add(left: u32, right: u32) -> u32 {
    common_add(left, right)
}

#[cfg(feature = "jsbindings")]
#[wasm_bindgen]
pub fn concatenate(left: &str, right: &str) -> String {
    common_concatenate(left, right)
}

#[cfg(feature = "jsbindings")]
#[wasm_bindgen]
pub fn parse_game_state(game_state_json: &str) -> bool {
    common_parse_game_state(game_state_json);
    true
}

#[cfg(feature = "pybindings")]
#[pyfunction]
pub fn concatenate(left: &str, right: &str) -> String {
    common_concatenate(left, right)
}

#[cfg(feature = "pybindings")]
#[pyfunction]
pub fn add(left: u32, right: u32) -> u32 {
    common_add(left, right)
}

#[cfg(feature = "pybindings")]
#[pyfunction]
pub fn parse_game_state(game_state_json: &str) -> bool {
    let game_state = common_parse_game_state(game_state_json);
    true
}

fn common_add(left: u32, right: u32) -> u32 {
    left + right
}

fn common_concatenate(left: &str, right: &str) -> String {
    let mut owned = left.to_string();
    owned.push_str(right);
    owned
}

/// A Python module implemented in Rust.
#[cfg(feature = "pybindings")]
#[pymodule]
fn rummy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(concatenate, m)?)?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(parse_game_state, m)?)?;
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
