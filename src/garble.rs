/// Position constraint for a garble dictionary entry.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Position {
    /// Matches at any position in the word.
    Any,
    /// Matches only at the start of the word (byte offset 0).
    Start,
    /// Matches only at the end of the word (key reaches last byte).
    End,
}

struct GarbleEntry {
    key: &'static str,
    replacement: &'static str,
    position: Position,
}

use Position::*;

/// All 43 garble phonetic dictionary entries.
/// Keys are lowercase; sorted by descending key length for greedy matching.
const GARBLE_PHRASES: &[GarbleEntry] = &[
    // 3-letter keys
    GarbleEntry {
        key: "aat",
        replacement: "aahtd",
        position: Any,
    },
    GarbleEntry {
        key: "ard",
        replacement: "ahrdt",
        position: Any,
    },
    GarbleEntry {
        key: "den",
        replacement: "dehn",
        position: Any,
    },
    GarbleEntry {
        key: "heb",
        replacement: "hebdt",
        position: Any,
    },
    GarbleEntry {
        key: "ijk",
        replacement: "ijck",
        position: Any,
    },
    GarbleEntry {
        key: "ijn",
        replacement: "ijhn",
        position: Any,
    },
    GarbleEntry {
        key: "nou",
        replacement: "nouwh",
        position: Any,
    },
    GarbleEntry {
        key: "oet",
        replacement: "oehdt",
        position: Any,
    },
    GarbleEntry {
        key: "tje",
        replacement: "tjuh",
        position: Any,
    },
    GarbleEntry {
        key: "van",
        replacement: "fahn",
        position: Any,
    },
    GarbleEntry {
        key: "ver",
        replacement: "fehr",
        position: Any,
    },
    GarbleEntry {
        key: "voo",
        replacement: "fooh",
        position: Any,
    },
    // 2-letter keys
    GarbleEntry {
        key: "aa",
        replacement: "aah",
        position: Any,
    },
    GarbleEntry {
        key: "ah",
        replacement: "ahdt",
        position: Any,
    },
    GarbleEntry {
        key: "as",
        replacement: "ahsz",
        position: Any,
    },
    GarbleEntry {
        key: "ch",
        replacement: "gch",
        position: Any,
    },
    GarbleEntry {
        key: "de",
        replacement: "deh",
        position: Any,
    },
    GarbleEntry {
        key: "dt",
        replacement: "dtdttd",
        position: Any,
    },
    GarbleEntry {
        key: "ee",
        replacement: "eeh",
        position: Any,
    },
    GarbleEntry {
        key: "eg",
        replacement: "egch",
        position: Any,
    },
    GarbleEntry {
        key: "el",
        replacement: "eehl",
        position: Any,
    },
    GarbleEntry {
        key: "en",
        replacement: "ehn",
        position: Any,
    },
    GarbleEntry {
        key: "et",
        replacement: "etd",
        position: Any,
    },
    GarbleEntry {
        key: "ie",
        replacement: "ieh",
        position: Any,
    },
    GarbleEntry {
        key: "ig",
        replacement: "igch",
        position: Any,
    },
    GarbleEntry {
        key: "ik",
        replacement: "icck",
        position: Any,
    },
    GarbleEntry {
        key: "il",
        replacement: "ihl",
        position: Any,
    },
    GarbleEntry {
        key: "in",
        replacement: "ihhn",
        position: Any,
    },
    GarbleEntry {
        key: "is",
        replacement: "ishj",
        position: Any,
    },
    GarbleEntry {
        key: "it",
        replacement: "iht",
        position: Any,
    },
    GarbleEntry {
        key: "ja",
        replacement: "jaah",
        position: Any,
    },
    GarbleEntry {
        key: "je",
        replacement: "jeh",
        position: Any,
    },
    GarbleEntry {
        key: "kk",
        replacement: "ckk",
        position: Any,
    },
    GarbleEntry {
        key: "me",
        replacement: "meh",
        position: Any,
    },
    GarbleEntry {
        key: "nu",
        replacement: "nuuh",
        position: Any,
    },
    GarbleEntry {
        key: "og",
        replacement: "ogch",
        position: Any,
    },
    GarbleEntry {
        key: "om",
        replacement: "ohmm",
        position: Any,
    },
    GarbleEntry {
        key: "oo",
        replacement: "oohw",
        position: Any,
    },
    GarbleEntry {
        key: "op",
        replacement: "ohp",
        position: Any,
    },
    GarbleEntry {
        key: "ov",
        replacement: "oowfv",
        position: Any,
    },
    GarbleEntry {
        key: "ro",
        replacement: "roow",
        position: Any,
    },
    GarbleEntry {
        key: "ys",
        replacement: "yszz",
        position: Any,
    },
    // 1-letter keys
    GarbleEntry {
        key: "z",
        replacement: "zs",
        position: Any,
    },
];

/// Replace all garble dictionary matches in `input` using a linear scanner.
/// Scans left-to-right, trying longest keys first at each position.
/// Respects position constraints (Start/End) on entries.
pub(crate) fn garble_replace_all(input: &str) -> String {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut result = String::with_capacity(input.len());
    let mut i = 0;
    while i < len {
        let mut matched = false;
        for entry in GARBLE_PHRASES {
            match entry.position {
                Any => {}
                Start => {
                    if i != 0 {
                        continue;
                    }
                }
                End => {
                    if i + entry.key.len() != len {
                        continue;
                    }
                }
            }
            if i + entry.key.len() <= len
                && bytes[i..i + entry.key.len()].eq_ignore_ascii_case(entry.key.as_bytes())
            {
                result.push_str(entry.replacement);
                i += entry.key.len();
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
                window[0].key.len() >= window[1].key.len(),
                "key '{}' (len {}) should come after '{}' (len {})",
                window[1].key,
                window[1].key.len(),
                window[0].key,
                window[0].key.len(),
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
        for entry in GARBLE_PHRASES {
            assert_eq!(
                garble_replace_all(entry.key),
                entry.replacement,
                "replacement for '{}' should be '{}'",
                entry.key,
                entry.replacement,
            );
        }
    }
}
