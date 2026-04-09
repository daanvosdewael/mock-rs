use std::sync::LazyLock;

#[cfg(not(feature = "wasm"))]
use regex::Regex;
#[cfg(feature = "wasm")]
use regex_lite::Regex;

/// All 43 garble phonetic dictionary entries.
/// Keys are lowercase; sorted by descending key length for greedy regex matching.
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

static GARBLE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    // Keys are already sorted by descending length in GARBLE_PHRASES,
    // so we can join them directly for greedy matching.
    let pattern: String = GARBLE_PHRASES
        .iter()
        .map(|(key, _)| *key)
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(&format!("(?i){pattern}")).expect("garble regex should compile")
});

pub(crate) fn garble_regex() -> &'static Regex {
    &GARBLE_REGEX
}

pub(crate) fn garble_replacement(matched: &str) -> Option<&'static str> {
    let lower = matched.to_lowercase();
    GARBLE_PHRASES
        .iter()
        .find(|(key, _)| *key == lower)
        .map(|(_, value)| *value)
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
    fn regex_matches_known_keys() {
        let re = garble_regex();
        assert!(re.is_match("aa"));
        assert!(re.is_match("ver"));
        assert!(re.is_match("z"));
    }

    #[test]
    fn regex_is_case_insensitive() {
        let re = garble_regex();
        assert!(re.is_match("AA"));
        assert!(re.is_match("Ver"));
        assert!(re.is_match("Z"));
    }

    #[test]
    fn regex_prefers_longer_match() {
        let re = garble_regex();
        // "aat" should match as "aat" (not "aa" + "t")
        let m = re.find("aat").unwrap();
        assert_eq!(m.as_str(), "aat");
    }

    #[test]
    fn lookup_returns_correct_replacement() {
        assert_eq!(garble_replacement("aa"), Some("aah"));
        assert_eq!(garble_replacement("aat"), Some("aahtd"));
        assert_eq!(garble_replacement("ver"), Some("fehr"));
        assert_eq!(garble_replacement("z"), Some("zs"));
    }

    #[test]
    fn lookup_is_case_insensitive() {
        assert_eq!(garble_replacement("AA"), Some("aah"));
        assert_eq!(garble_replacement("Ver"), Some("fehr"));
    }

    #[test]
    fn lookup_returns_none_for_unknown() {
        assert_eq!(garble_replacement("xyz"), None);
        assert_eq!(garble_replacement(""), None);
    }

    #[test]
    fn all_entries_are_matchable() {
        let re = garble_regex();
        for (key, _) in GARBLE_PHRASES {
            assert!(re.is_match(key), "regex should match key '{key}'");
        }
    }

    #[test]
    fn all_entries_have_replacements() {
        for (key, expected) in GARBLE_PHRASES {
            assert_eq!(
                garble_replacement(key),
                Some(*expected),
                "replacement for '{key}' should be '{expected}'"
            );
        }
    }
}
