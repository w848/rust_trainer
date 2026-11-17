`if` is an **expression** in Rust: it gives back a value.

```rust
let size = if n > 100 { "big" } else { "small" };
```

Every branch must give the **same type**.

**Your task:** write `fizzbuzz(n: u32) -> String`:

- `"FizzBuzz"` if `n` is divisible by 3 **and** by 5
- `"Fizz"` if divisible by 3
- `"Buzz"` if divisible by 5
- otherwise the number itself, as text: `"7"`
