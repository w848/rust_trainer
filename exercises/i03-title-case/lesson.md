Some text tools you will use all the time:

```rust
"  the  rust book ".split_whitespace()   // "the", "rust", "book"
vec!["a", "b"].join("-")                 // "a-b"

let mut chars = "rust".chars();
chars.next()          // Some('r'): takes the first letter
chars.as_str()        // "ust": what's left
'r'.to_uppercase()    // an iterator, .collect::<String>() makes "R"
```

To collect iterator results into a `Vec<String>`, use `.collect::<Vec<String>>()`
(or `.collect::<Vec<_>>()` and let Rust guess the inside type).

**Your tasks:**

1. `capitalize(word)`: first letter in capital, the rest in small letters. `"rUST"` → `"Rust"`.
   An empty word gives `""`.
2. `title_case(sentence)`: capitalize every word, separated by one space.
   `"  the rust   book "` → `"The Rust Book"`.
