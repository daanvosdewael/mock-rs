mod alternate;
mod correct;
mod dress;
mod phrases;

pub use alternate::alternate_word;
pub use correct::correct_word;
pub use dress::dress_word;

pub fn build_mocking_text(input: &str, garble: bool) -> String {
    let _ = garble;
    input.to_string()
}
