`if` picks one of two ways depending on a `bool`:

```rust
fn absolute(n: i32) -> i32 {
    if n < 0 {
        -n
    } else {
        n
    }
}
```

In Rust, `if ... else` **gives back a value**, so it can be the last line of a function.
No `;` after the values inside.

**Your task:** `bigger(a, b)` gives back the bigger of the two numbers.
`bigger(3, 8)` must give `8`.
