# mock-rs domain context

Living glossary for the mock-rs crate. Update inline when terms sharpen or new concepts get a name.

## Specials

The set `{ e, i, l, o }` (case-insensitive). Each special character has a weight:

| char | weight |
|------|--------|
| `e`  |   2    |
| `i`  |   3    |
| `l`  |  -2    |
| `o`  |   2    |

The set drives two stage decisions:

- **`alternate` stage** — sums the weights of specials at even vs. odd positions in a word. The sign of `(even_sum - odd_sum)` determines which positions get uppercased ("case policy" for the word).
- **`correct` stage** — when a special character is encountered in the alternated output, the special and its immediate neighbors are case-rewritten. `l` inverts the rule (its neighbors become *lower* and the `l` itself becomes *upper*); `e`/`i`/`o` apply the non-inverted rule.

Defined once in `src/specials.rs` as `is_special(c) -> bool` and `special_weight(c) -> i32`. Both `alternate` and `correct` consume it; no other module knows the set.

## Pipeline

The three transformation stages, in order:

```
garble (optional) → alternate → correct
```

- **`garble_word`** (`src/garble.rs`) — phonetic substitution from a 58-entry dictionary. Only runs when the `garble` flag is set. Lowercases the word, applies greedy left-to-right matching with position constraints (`Start`/`End`/`Any`), returns the original case if no match.
- **`alternate_word`** (`src/alternate.rs`) — alternates the case of each character based on the case policy computed from the specials in the word.
- **`correct_word`** (`src/correct.rs`) — post-corrects the alternated output by rewriting each special character and its neighbors.

The order is load-bearing: `correct` assumes its input has been alternated (its tests are named `*_alternated` for that reason), and `alternate` assumes its input is the original word (or the garbled word, if garble is on). `lib.rs::build_mocking_text` is the only place the pipeline is wired up.
