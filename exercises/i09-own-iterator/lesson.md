Any type can become an iterator: implement the `Iterator` trait.
You only write `next()`; `take`, `filter`, `sum`, `collect`... come for free!

```rust
struct Countdown { left: u32 }

impl Iterator for Countdown {
    type Item = u32;                       // what it gives
    fn next(&mut self) -> Option<u32> {
        if self.left == 0 {
            None                           // None = finished
        } else {
            self.left -= 1;
            Some(self.left + 1)
        }
    }
}
// Countdown { left: 3 }.collect::<Vec<_>>() == vec![3, 2, 1]
```

**Your tasks:**

1. `Fibonacci::new()` and `next()`: it gives 0, 1, 1, 2, 3, 5, 8, ... forever
   (each number is the sum of the two before).
2. `even_sum_below(limit)`: the sum of the **even** Fibonacci numbers smaller than `limit`.
   Use `take_while` and `filter` on your iterator.
