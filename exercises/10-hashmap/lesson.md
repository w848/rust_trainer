`HashMap<K, V>` stores values by key:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("blue", 10);
println!("{:?}", scores.get("blue")); // Some(10)
```

The **entry API** inserts a default value if the key is missing, then lets you
change it:

```rust
*scores.entry("red").or_insert(0) += 5;
```

**Your task:** write `word_count(text: &str) -> HashMap<String, usize>` that counts
each word. Words are separated by whitespace and counted **without case**:
`"The cat the"` gives `{"the": 2, "cat": 1}`.
