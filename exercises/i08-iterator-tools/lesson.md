Three iterator tools that replace a lot of loops:

```rust
// filter_map: keep only the Some values (and unwrap them)
["1", "x", "3"].iter().filter_map(|s| s.parse::<i32>().ok())   // 1, 3

// zip: walk two lists side by side, as pairs
[1, 2].iter().zip([10, 20].iter())                              // (1,10), (2,20)

// enumerate: get the position with each element
["a", "b"].iter().enumerate()                                   // (0,"a"), (1,"b")
```

`.ok()` turns a `Result` into an `Option`. Try to write each function
**without a `for` loop**, in one chain.

**Your tasks:**

1. `sum_valid(inputs)`: the sum of the texts that are valid numbers (spaces allowed around).
2. `dot_product(a, b)`: `a[0]*b[0] + a[1]*b[1] + ...` (stop at the shorter list).
3. `positions_of(items, target)`: the positions where `target` appears.
