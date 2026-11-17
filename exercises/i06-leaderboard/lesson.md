`v.sort()` works for simple things. To sort by your own rule, use `sort_by`
with a closure that compares two elements and returns an `Ordering`:

```rust
let mut words = vec!["pear", "fig", "banana"];
words.sort_by(|a, b| a.len().cmp(&b.len()));   // shortest first
words.sort_by(|a, b| b.len().cmp(&a.len()));   // swap a and b: longest first
```

To break ties, chain with `.then(...)`:

```rust
a.len().cmp(&b.len()).then(a.cmp(b))   // by length, then alphabetically
```

**Your tasks:**

1. `rank(players)`: sort by score, **highest first**. Same score → alphabetical by name.
2. `podium(players)`: the names of the top 3 (fewer if there are fewer players).
