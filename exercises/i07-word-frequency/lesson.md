Let's combine `HashMap` and sorting into a little text analyser.

```rust
"Hello,".trim_matches(|c: char| !c.is_alphanumeric())   // "Hello"
```

`trim_matches` removes characters from both ends while the closure says `true`.

A `HashMap` has no order. To sort it, turn it into a `Vec` of pairs first:

```rust
let mut pairs: Vec<(String, usize)> = map.into_iter().collect();
pairs.sort_by(|a, b| b.1.cmp(&a.1));   // .0 is the word, .1 the count
pairs.truncate(3);                     // keep only the first 3
```

**Your tasks:**

1. `word_counts(text)`: how many times each word appears. Words are lowercased and
   punctuation at their ends is removed (`"Rust!"` and `"rust"` are the same word).
2. `top_words(text, n)`: the `n` most used words with their count, most used first,
   ties in alphabetical order.
