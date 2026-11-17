With `else if` you can test several things, one after the other:

```rust
fn grade(points: i32) -> i32 {
    if points >= 90 {
        3
    } else if points >= 50 {
        2
    } else {
        1
    }
}
```

Rust checks from top to bottom and stops at the first one that is `true`.

**Your task:** `sign(n)` gives back:

- `1` when `n` is positive,
- `-1` when `n` is negative,
- `0` when `n` is zero.
