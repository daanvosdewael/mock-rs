use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn mock(input: &str, garble: bool) -> String {
    mock_core::build_mocking_text(input, garble)
}
