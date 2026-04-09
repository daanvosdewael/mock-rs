fn special_weight(c: char) -> i32 {
    match c.to_ascii_lowercase() {
        'e' => 2,
        'i' => 3,
        'l' => -2,
        'o' => 2,
        _ => 0,
    }
}

fn is_special(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'e' | 'i' | 'l' | 'o')
}

fn should_uppercase(word: &str, char_index: usize) -> bool {
    let mut even_index_sum: i32 = 0;
    let mut odd_index_sum: i32 = 0;

    for (i, c) in word.chars().enumerate() {
        if !is_special(c) {
            continue;
        }
        if i % 2 == 0 {
            even_index_sum += special_weight(c);
        } else {
            odd_index_sum += special_weight(c);
        }
    }

    if even_index_sum < odd_index_sum {
        char_index.is_multiple_of(2)
    } else {
        !char_index.is_multiple_of(2)
    }
}

pub fn alternate_word(word: &str) -> String {
    word.chars()
        .enumerate()
        .map(|(i, c)| {
            if should_uppercase(word, i) {
                c.to_uppercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert_eq!(alternate_word(""), "");
    }

    #[test]
    fn single_non_special_char() {
        // No specials → even >= odd → uppercase at odd positions
        // Index 0 is even → not uppercase
        assert_eq!(alternate_word("a"), "a");
        assert_eq!(alternate_word("x"), "x");
    }

    #[test]
    fn single_special_char() {
        // Single special at index 0: even_sum = weight, odd_sum = 0
        // e: even(2) >= odd(0) → uppercase at odd → "e"
        assert_eq!(alternate_word("e"), "e");
        // i: even(3) >= odd(0) → uppercase at odd → "i"
        assert_eq!(alternate_word("i"), "i");
        // l: even(-2) < odd(0) → uppercase at even → "L"
        assert_eq!(alternate_word("l"), "L");
        // o: even(2) >= odd(0) → uppercase at odd → "o"
        assert_eq!(alternate_word("o"), "o");
    }

    #[test]
    fn no_specials_uppercase_at_odd() {
        // even_sum=0, odd_sum=0 → even >= odd → uppercase at odd positions
        assert_eq!(alternate_word("xy"), "xY");
        assert_eq!(alternate_word("xyz"), "xYz");
        assert_eq!(alternate_word("xyzw"), "xYzW");
    }

    #[test]
    fn all_same_special_e() {
        // "eeeee": even indices (0,2,4) sum = 6, odd indices (1,3) sum = 4
        // even(6) >= odd(4) → uppercase at odd positions
        assert_eq!(alternate_word("eeeee"), "eEeEe");
    }

    #[test]
    fn all_same_special_l() {
        // "lllll": even indices (0,2,4) sum = -6, odd indices (1,3) sum = -4
        // even(-6) < odd(-4) → uppercase at even positions
        assert_eq!(alternate_word("lllll"), "LlLlL");
    }

    #[test]
    fn all_same_special_i() {
        // "iiiii": even (0,2,4) = 9, odd (1,3) = 6
        // even(9) >= odd(6) → uppercase at odd
        assert_eq!(alternate_word("iiiii"), "iIiIi");
    }

    #[test]
    fn all_same_special_o() {
        // "ooooo": even (0,2,4) = 6, odd (1,3) = 4
        // even(6) >= odd(4) → uppercase at odd
        assert_eq!(alternate_word("ooooo"), "oOoOo");
    }

    #[test]
    fn test_before_correct() {
        // "test": e at index 1 → even=0, odd=2
        // even(0) < odd(2) → uppercase at even positions
        // T(0) e(1) S(2) t(3) → "TeSt"
        assert_eq!(alternate_word("test"), "TeSt");
    }

    #[test]
    fn foobar_before_correct() {
        // "foobar": even indices: f(0)=0, o(2)=2, a(4)=0 → 2
        //          odd indices:  o(1)=2, b(3)=0, r(5)=0 → 2
        // even(2) >= odd(2) → uppercase at odd
        assert_eq!(alternate_word("foobar"), "fOoBaR");
    }

    #[test]
    fn mixed_case_input_preserves_non_uppercase() {
        // "HELLO": uppercase at odd (even=0, odd=0 for non-specials h,no wait)
        // h(0)=0, e(1)=2, l(2)=-2, l(3)=-2, o(4)=2
        // even: e-weight at 0? no, h is not special. l at 2 = -2, o at 4 = 2 → 0
        // odd: e at 1 = 2, l at 3 = -2 → 0
        // even(0) >= odd(0) → uppercase at odd positions
        // H(0)→H, E(1)→E, L(2)→L, L(3)→L, O(4)→O → "HELLO"
        assert_eq!(alternate_word("HELLO"), "HELLO");
    }

    #[test]
    fn lowercase_hello() {
        // "hello": same weight calc as HELLO → uppercase at odd
        // h(0), e(1)→E, l(2), l(3)→L, o(4) → "hElLo"
        assert_eq!(alternate_word("hello"), "hElLo");
    }

    #[test]
    fn even_less_than_odd_uppercase_at_even() {
        // Need a word where even_index_sum < odd_index_sum
        // "le": l at 0 = -2, e at 1 = 2 → even=-2, odd=2 → even < odd → uppercase at even
        assert_eq!(alternate_word("le"), "Le");
    }

    #[test]
    fn unicode_passthrough() {
        // Non-ASCII chars are not special, just alternate normally
        assert_eq!(alternate_word("ab"), "aB");
        // "über": e at index 2 → even=2, odd=0 → uppercase at odd
        assert_eq!(alternate_word("über"), "üBeR");
    }

    #[test]
    fn numbers_and_punctuation() {
        assert_eq!(alternate_word("123"), "123");
        // "a1b": no specials → even>=odd → uppercase at odd → a(0)1(1)b(2) = "a1b"
        assert_eq!(alternate_word("a1b"), "a1b");
    }
}
