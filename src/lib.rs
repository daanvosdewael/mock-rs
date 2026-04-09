mod alternate;
mod garble;
mod specials;

pub use alternate::alternate_word;
pub use garble::garble_word;
pub use specials::apply_specials;

pub fn build_mocking_text(input: &str, garble: bool) -> String {
    input
        .split(' ')
        .map(|word| {
            if word.is_empty() {
                return String::new();
            }
            let dressed = if garble {
                garble_word(word)
            } else {
                word.to_string()
            };
            let alternated = alternate_word(&dressed);
            apply_specials(&alternated)
        })
        .collect::<Vec<_>>()
        .join(" ")
}
