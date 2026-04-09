use crate::garble::garble_replace_all;

pub fn dress_word(word: &str) -> String {
    let lower = word.to_lowercase();
    let replaced = garble_replace_all(&lower);
    if replaced == lower {
        word.to_string()
    } else {
        replaced
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
