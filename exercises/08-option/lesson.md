Rust has **no null**. A value that may be missing is an `Option<T>`:

```rust
enum Option<T> {
    Some(T),  // there is a value
    None,     // there isn't
}
```

You must handle both cases before using the value:

```rust
match find_user(id) {
    Some(user) => println!("found {user}"),
    None => println!("nobody"),
}
// or: if let Some(user) = find_user(id) { ... }
```

**Your task:** write `find_index(numbers: &[i32], target: i32) -> Option<usize>`
that returns the position of the **first** `target`, or `None` if it isn't there.
`find_index(&[5, 8, 8], 8)` is `Some(1)`.
