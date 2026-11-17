Iterators let you chain operations instead of writing loops:

```rust
let v = vec![1, 2, 3, 4];
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();   // [2, 4, 6, 8]
let big: Vec<&i32> = v.iter().filter(|x| **x > 2).collect(); // [3, 4]
let total: i32 = v.iter().sum();                             // 10
```

`|x| x * 2` is a **closure**: a small function written inline.
Iterators are lazy: nothing happens until `collect()`, `sum()`, a `for` loop...

**Your task:** write `sum_of_odd_squares(numbers: &[i32]) -> i32` that keeps the
**odd** numbers, squares them, and adds them up, using iterators (no `for`).
`[1, 2, 3]` gives `1² + 3² = 10`.
