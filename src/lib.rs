mod alternate;
mod correct;
mod dress;
mod garble;

pub use alternate::alternate_word;
pub use correct::correct_word;
pub use dress::dress_word;

pub fn build_mocking_text(input: &str, garble: bool) -> String {
    input
        .split(' ')
        .map(|word| {
            if word.is_empty() {
                return String::new();
            }
            let dressed = if garble {
                dress_word(word)
            } else {
                word.to_string()
            };
            let alternated = alternate_word(&dressed);
            correct_word(&alternated)
        })
        .collect::<Vec<_>>()
        .join(" ")
}
