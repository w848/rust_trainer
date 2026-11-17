`while` repeats its block as long as the condition is `true`:

```rust
let mut count = 0;
while count < 3 {
    println!("count is {count}");
    count += 1;     // don't forget this, or it never stops!
}
```

This prints `count is 0`, `count is 1`, `count is 2`.

**Your task:** `sum_to(n)` adds all the numbers from 1 to `n`.
`sum_to(4)` must give `10`  (1 + 2 + 3 + 4).

Most of the code is there. Fill in the inside of the loop.
(If you forget `i += 1`, the loop never ends: the trainer stops it after 5 seconds.)

Before you fill it in, the compiler warns `variable does not need to be mutable`:
that's normal, the warning goes away once your loop changes `sum` and `i`.
