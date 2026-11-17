When a function **returns a reference**, Rust must know how long it stays valid.
If it could come from either parameter, you name a **lifetime** `'a`:

```rust
fn pick<'a>(a: &'a str, b: &'a str) -> &'a str { a }
```

`'a` says: the result lives as long as **both** inputs do. The compiler then
refuses code that would use the result after one of the inputs is gone.

Try it first **without** `<'a>`: read the compiler's error message, it explains
exactly what's missing. Rust's error messages are very good teachers.

**Your task:** write `longest` that returns the longer of two texts.
If they have the same length, return the **first** one.
