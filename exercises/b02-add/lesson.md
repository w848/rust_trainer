Functions can receive values. They are called **parameters**, and each one has a type:

```rust
fn times_two(n: i32) -> i32 {
    n * 2
}
```

`times_two(5)` gives `10`. The math signs are the usual ones:
`+` add, `-` subtract, `*` multiply, `/` divide.

**Your task:** write `add(a, b)` so it gives back `a` plus `b`.
`add(2, 3)` must give `5`.
