A single letter has the type `char` and uses **single** quotes: `'a'`.
(Double quotes `"a"` make text, not a letter.)

`.chars()` lets a `for` loop go through the letters of a text:

```rust
for c in "cat".chars() {
    println!("{c}");   // c, then a, then t
}
```

You can put an `if` inside a loop.

**Your task:** `count_letter(text, letter)` gives back how many times `letter` is in `text`.
`count_letter("banana", 'a')` must give `3`.

This one uses everything from before: `let mut`, `for`, `if`. You can do it!
After this one, you're ready for the **Practice** exercises.
