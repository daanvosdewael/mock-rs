//! The set of special characters (`e`, `i`, `l`, `o`) and their position weights.
//!
//! Both the `alternate` stage (for the case-policy decision) and the `correct`
//! stage (for the per-position rewrite) key on this set.

pub fn is_special(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'e' | 'i' | 'l' | 'o')
}

pub fn special_weight(c: char) -> i32 {
    match c.to_ascii_lowercase() {
        'e' => 2,
        'i' => 3,
        'l' => -2,
        'o' => 2,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_special_recognises_lowercase_set() {
        assert!(is_special('e'));
        assert!(is_special('i'));
        assert!(is_special('l'));
        assert!(is_special('o'));
    }

    #[test]
    fn is_special_recognises_uppercase_set() {
        assert!(is_special('E'));
        assert!(is_special('I'));
        assert!(is_special('L'));
        assert!(is_special('O'));
    }

    #[test]
    fn is_special_rejects_other_chars() {
        assert!(!is_special('a'));
        assert!(!is_special('z'));
        assert!(!is_special('1'));
        assert!(!is_special(' '));
        assert!(!is_special('!'));
        assert!(!is_special('ü'));
    }

    #[test]
    fn special_weight_table() {
        assert_eq!(special_weight('e'), 2);
        assert_eq!(special_weight('i'), 3);
        assert_eq!(special_weight('l'), -2);
        assert_eq!(special_weight('o'), 2);
    }

    #[test]
    fn special_weight_is_case_insensitive() {
        assert_eq!(special_weight('E'), 2);
        assert_eq!(special_weight('I'), 3);
        assert_eq!(special_weight('L'), -2);
        assert_eq!(special_weight('O'), 2);
    }

    #[test]
    fn special_weight_zero_for_non_specials() {
        assert_eq!(special_weight('a'), 0);
        assert_eq!(special_weight('z'), 0);
        assert_eq!(special_weight('1'), 0);
        assert_eq!(special_weight(' '), 0);
    }
}
