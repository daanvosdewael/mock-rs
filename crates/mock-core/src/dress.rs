use crate::phrases::{garble_regex, garble_replacement};

#[cfg(not(feature = "wasm"))]
use regex::Captures;
#[cfg(feature = "wasm")]
use regex_lite::Captures;

pub fn dress_word(word: &str) -> String {
    let re = garble_regex();
    let lower = word.to_lowercase();
    if re.is_match(&lower) {
        re.replace_all(&lower, |caps: &Captures| {
            garble_replacement(&caps[0]).unwrap_or(&caps[0]).to_string()
        })
        .into_owned()
    } else {
        word.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_matching_word_unchanged() {
        assert_eq!(dress_word("test"), "test");
        assert_eq!(dress_word("abc"), "abc");
        assert_eq!(dress_word(""), "");
    }

    #[test]
    fn matching_word_replaced() {
        assert_eq!(dress_word("aa"), "aah");
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(dress_word("AA"), "aah");
        assert_eq!(dress_word("Aa"), "aah");
        assert_eq!(dress_word("HELLO"), "heehllo");
    }

    #[test]
    fn longer_pattern_priority() {
        assert_eq!(dress_word("aat"), "aahtd");
    }

    #[test]
    fn key_in_word() {
        assert_eq!(dress_word("hello"), "heehllo");
        assert_eq!(dress_word("foobar"), "foohwbar");
    }

    #[test]
    fn multiple_replacements() {
        assert_eq!(dress_word("aaz"), "aahzs");
    }
}
