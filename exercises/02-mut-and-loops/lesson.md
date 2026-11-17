Variables are **immutable** by default. Add `mut` to change them:

```rust
let x = 5;      // can't change
let mut y = 5;  // can change
y += 1;
```

A `for` loop walks over a **range**:

- `1..5` is 1, 2, 3, 4 (5 excluded)
- `1..=5` is 1, 2, 3, 4, 5 (5 included)

**Your task:** write `sum_to(n: u32) -> u32` that adds all numbers from 1 to `n`.
`sum_to(4)` is `1 + 2 + 3 + 4 = 10`, and `sum_to(0)` is `0`.
