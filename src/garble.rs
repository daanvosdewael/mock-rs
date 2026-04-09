/// All 43 garble phonetic dictionary entries.
/// Keys are lowercase; sorted by descending key length for greedy matching.
const GARBLE_PHRASES: &[(&str, &str)] = &[
    ("aat", "aahtd"),
    ("ard", "ahrdt"),
    ("den", "dehn"),
    ("heb", "hebdt"),
    ("ijk", "ijck"),
    ("ijn", "ijhn"),
    ("nou", "nouwh"),
    ("oet", "oehdt"),
    ("tje", "tjuh"),
    ("van", "fahn"),
    ("ver", "fehr"),
    ("voo", "fooh"),
    ("aa", "aah"),
    ("ah", "ahdt"),
    ("as", "ahsz"),
    ("ch", "gch"),
    ("de", "deh"),
    ("dt", "dtdttd"),
    ("ee", "eeh"),
    ("eg", "egch"),
    ("el", "eehl"),
    ("en", "ehn"),
    ("et", "etd"),
    ("ie", "ieh"),
    ("ig", "igch"),
    ("ik", "icck"),
    ("il", "ihl"),
    ("in", "ihhn"),
    ("is", "ishj"),
    ("it", "iht"),
    ("ja", "jaah"),
    ("je", "jeh"),
    ("kk", "ckk"),
    ("me", "meh"),
    ("nu", "nuuh"),
    ("og", "ogch"),
    ("om", "ohmm"),
    ("oo", "oohw"),
    ("op", "ohp"),
    ("ov", "oowfv"),
    ("ro", "roow"),
    ("ys", "yszz"),
    ("z", "zs"),
];

/// Replace all garble dictionary matches in `input` using a linear scanner.
/// Scans left-to-right, trying longest keys first at each position.
pub(crate) fn garble_replace_all(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut result = String::with_capacity(input.len());
    let mut i = 0;
    while i < bytes.len() {
        let mut matched = false;
        for &(key, replacement) in GARBLE_PHRASES {
            if i + key.len() <= bytes.len()
                && bytes[i..i + key.len()].eq_ignore_ascii_case(key.as_bytes())
            {
                result.push_str(replacement);
                i += key.len();
                matched = true;
                break;
            }
        }
        if !matched {
            // Advance by one full UTF-8 character, not just one byte.
            let ch = &input[i..];
            let c = ch.chars().next().unwrap();
            result.push(c);
            i += c.len_utf8();
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dictionary_has_43_entries() {
        assert_eq!(GARBLE_PHRASES.len(), 43);
    }

    #[test]
    fn keys_sorted_by_descending_length() {
        for window in GARBLE_PHRASES.windows(2) {
            assert!(
                window[0].0.len() >= window[1].0.len(),
                "key '{}' (len {}) should come after '{}' (len {})",
                window[1].0,
                window[1].0.len(),
                window[0].0,
                window[0].0.len(),
            );
        }
    }

    #[test]
    fn replaces_known_keys() {
        assert_eq!(garble_replace_all("aa"), "aah");
        assert_eq!(garble_replace_all("ver"), "fehr");
        assert_eq!(garble_replace_all("z"), "zs");
    }

    #[test]
    fn case_insensitive() {
        assert_eq!(garble_replace_all("AA"), "aah");
        assert_eq!(garble_replace_all("Ver"), "fehr");
        assert_eq!(garble_replace_all("Z"), "zs");
    }

    #[test]
    fn prefers_longer_match() {
        assert_eq!(garble_replace_all("aat"), "aahtd");
    }

    #[test]
    fn no_match_returns_unchanged() {
        assert_eq!(garble_replace_all("xy"), "xy");
        assert_eq!(garble_replace_all(""), "");
        assert_eq!(garble_replace_all("b"), "b");
    }

    #[test]
    fn multiple_replacements() {
        assert_eq!(garble_replace_all("aaz"), "aahzs");
    }

    #[test]
    fn all_entries_replace_correctly() {
        for (key, expected) in GARBLE_PHRASES {
            assert_eq!(
                garble_replace_all(key),
                *expected,
                "replacement for '{key}' should be '{expected}'"
            );
        }
    }
}
