fn is_special(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'e' | 'i' | 'l' | 'o')
}

fn apply_case(c: char, special: char, invert: bool) -> char {
    let is_l = special == 'l';
    if is_l ^ invert {
        c.to_ascii_uppercase()
    } else {
        c.to_ascii_lowercase()
    }
}

pub fn apply_specials(word: &str) -> String {
    if !word.chars().any(is_special) {
        return word.to_string();
    }

    let chars: Vec<char> = word.chars().collect();
    let mut result: Vec<char> = Vec::with_capacity(chars.len());
    let mut next_up = false;

    for (i, &ch) in chars.iter().enumerate() {
        if next_up {
            next_up = false;
            if is_special(ch) {
                result.pop();
                result.push(apply_case(ch, ch, false));
            }
            continue;
        }

        if !is_special(ch) {
            result.push(ch);
            continue;
        }

        next_up = true;
        let last = result.pop();
        let next = chars.get(i + 1).copied();

        if let Some(last) = last {
            result.push(apply_case(last, ch, true));
        }
        result.push(apply_case(ch, ch, false));
        if let Some(next) = next {
            result.push(apply_case(next, ch, true));
        }
    }

    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert_eq!(apply_specials(""), "");
    }

    #[test]
    fn no_specials() {
        assert_eq!(apply_specials("abc"), "abc");
        assert_eq!(apply_specials("xYzW"), "xYzW");
    }

    #[test]
    fn single_special_chars() {
        assert_eq!(apply_specials("e"), "e");
        assert_eq!(apply_specials("i"), "i");
        assert_eq!(apply_specials("l"), "L");
        assert_eq!(apply_specials("o"), "o");
    }

    #[test]
    fn single_special_uppercase() {
        assert_eq!(apply_specials("E"), "e");
        assert_eq!(apply_specials("I"), "i");
        assert_eq!(apply_specials("L"), "l");
        assert_eq!(apply_specials("O"), "o");
    }

    #[test]
    fn hello_alternated() {
        // alternate_word("hello") = "hElLo" → correct → "HeLlo"
        assert_eq!(apply_specials("hElLo"), "HeLlo");
    }

    #[test]
    fn lllll_alternated() {
        // alternate_word("lllll") = "LlLlL" → correct → "lLlLl"
        assert_eq!(apply_specials("LlLlL"), "lLlLl");
    }

    #[test]
    fn eeeee_alternated() {
        // alternate_word("eeeee") = "eEeEe" → correct → "eEeEe"
        assert_eq!(apply_specials("eEeEe"), "eEeEe");
    }

    #[test]
    fn special_at_start_with_next() {
        // 'e' at 0: no prev, self→lower, next→upper
        assert_eq!(apply_specials("eA"), "eA");
        assert_eq!(apply_specials("ea"), "eA");
    }

    #[test]
    fn special_at_end() {
        // 'E' at 1: prev→upper, self→lower
        assert_eq!(apply_specials("aE"), "Ae");
    }

    #[test]
    fn consecutive_specials() {
        assert_eq!(apply_specials("eL"), "el");
    }

    #[test]
    fn foobar_alternated() {
        // alternate_word("foobar") = "fOoBaR" → correct → "FooBaR"
        assert_eq!(apply_specials("fOoBaR"), "FooBaR");
    }

    #[test]
    fn test_alternated() {
        // alternate_word("test") = "TeSt" → correct → "TeSt"
        assert_eq!(apply_specials("TeSt"), "TeSt");
    }

    #[test]
    fn numbers_and_punctuation() {
        assert_eq!(apply_specials("123"), "123");
        assert_eq!(apply_specials("!@#"), "!@#");
    }

    #[test]
    fn l_lowercase_vs_uppercase_differ() {
        // lowercase 'l': self→upper, surrounding→lower
        // uppercase 'L': self→lower, surrounding→upper (like e/i/o)
        assert_eq!(apply_specials("alb"), "aLb");
        assert_eq!(apply_specials("aLb"), "AlB");
    }
}
