Rust has **no exceptions**. A function that can fail returns a `Result`:

```rust
enum Result<T, E> {
    Ok(T),   // success, with the value
    Err(E),  // failure, with the error
}
```

`"42".parse::<u8>()` returns `Ok(42)`, and `"abc".parse::<u8>()` returns an `Err`.

Inside a function that returns a `Result`, the **`?` operator** returns the error
early: `let n = text.parse::<u8>()?;`.

**Your task:** write `parse_age(s: &str) -> Result<u8, String>`:

- ignore spaces around the text
- `"42"` gives `Ok(42)`
- text that isn't a number in 0–255 gives `Err("not a number")`
- a number above 150 gives `Err("too old")`
