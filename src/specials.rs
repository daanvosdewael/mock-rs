//! The set of special characters (`e`, `i`, `l`, `o`) and the case policy
//! they imply for a word.
//!
//! - `alternate` consumes [`case_policy`] to decide which positions get
//!   uppercased.
//! - `correct` consumes [`is_special`] (membership in the set).

pub fn is_special(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'e' | 'i' | 'l' | 'o')
}

/// Which position class (even or odd) gets uppercased in a word.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CasePolicy {
    UpperEven,
    UpperOdd,
}

impl CasePolicy {
    pub fn is_upper_at(self, index: usize) -> bool {
        match self {
            CasePolicy::UpperEven => index.is_multiple_of(2),
            CasePolicy::UpperOdd => !index.is_multiple_of(2),
        }
    }
}

/// Case policy for a word, derived from the weights of its specials.
///
/// `UpperEven` when the even-position weight sum is strictly less than the
/// odd-position weight sum; `UpperOdd` otherwise (including ties and words
/// with no specials).
pub fn case_policy(word: &str) -> CasePolicy {
    let mut even_sum: i32 = 0;
    let mut odd_sum: i32 = 0;
    for (i, c) in word.chars().enumerate() {
        if !is_special(c) {
            continue;
        }
        if i.is_multiple_of(2) {
            even_sum += special_weight(c);
        } else {
            odd_sum += special_weight(c);
        }
    }
    if even_sum < odd_sum {
        CasePolicy::UpperEven
    } else {
        CasePolicy::UpperOdd
    }
}

fn special_weight(c: char) -> i32 {
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

    #[test]
    fn upper_even_at_even_indices() {
        assert!(CasePolicy::UpperEven.is_upper_at(0));
        assert!(!CasePolicy::UpperEven.is_upper_at(1));
        assert!(CasePolicy::UpperEven.is_upper_at(2));
        assert!(!CasePolicy::UpperEven.is_upper_at(3));
    }

    #[test]
    fn upper_odd_at_odd_indices() {
        assert!(!CasePolicy::UpperOdd.is_upper_at(0));
        assert!(CasePolicy::UpperOdd.is_upper_at(1));
        assert!(!CasePolicy::UpperOdd.is_upper_at(2));
        assert!(CasePolicy::UpperOdd.is_upper_at(3));
    }

    #[test]
    fn case_policy_empty_is_upper_odd() {
        assert_eq!(case_policy(""), CasePolicy::UpperOdd);
    }

    #[test]
    fn case_policy_no_specials_is_upper_odd() {
        assert_eq!(case_policy("abc"), CasePolicy::UpperOdd);
        assert_eq!(case_policy("xyz"), CasePolicy::UpperOdd);
        assert_eq!(case_policy("123"), CasePolicy::UpperOdd);
    }

    #[test]
    fn case_policy_single_special() {
        assert_eq!(case_policy("e"), CasePolicy::UpperOdd);
        assert_eq!(case_policy("i"), CasePolicy::UpperOdd);
        assert_eq!(case_policy("l"), CasePolicy::UpperEven);
        assert_eq!(case_policy("o"), CasePolicy::UpperOdd);
    }

    #[test]
    fn case_policy_all_same_special() {
        assert_eq!(case_policy("eeeee"), CasePolicy::UpperOdd);
        assert_eq!(case_policy("iiiii"), CasePolicy::UpperOdd);
        assert_eq!(case_policy("lllll"), CasePolicy::UpperEven);
        assert_eq!(case_policy("ooooo"), CasePolicy::UpperOdd);
    }

    #[test]
    fn case_policy_mixed_words() {
        assert_eq!(case_policy("test"), CasePolicy::UpperEven);
        assert_eq!(case_policy("foobar"), CasePolicy::UpperOdd);
        assert_eq!(case_policy("le"), CasePolicy::UpperEven);
        assert_eq!(case_policy("hello"), CasePolicy::UpperOdd);
    }

    #[test]
    fn case_policy_case_insensitive() {
        assert_eq!(case_policy("HELLO"), case_policy("hello"));
        assert_eq!(case_policy("EILO"), case_policy("eilo"));
    }

    #[test]
    fn case_policy_unicode_passthrough() {
        assert_eq!(case_policy("über"), CasePolicy::UpperOdd);
    }
}
