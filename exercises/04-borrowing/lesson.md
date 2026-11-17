**Ownership** is Rust's big idea. Each value has one owner; when the owner
goes away, the value is freed. To use a value without taking it, you
**borrow** it with `&`:

```rust
fn length(s: &String) -> usize { s.len() } // borrows, doesn't take
let name = String::from("Ferris");
let n = length(&name);   // name is still usable after this
```

A function can return a **slice**: a view into part of the borrowed text,
with no copy. `&s[0..3]` is the first 3 bytes of `s`.

**Your task:** write `first_word(s: &str) -> &str` that returns the first word
(text up to the first space). Leading spaces are ignored.
`first_word("hello world")` is `"hello"`. For an empty text, return `""`.
