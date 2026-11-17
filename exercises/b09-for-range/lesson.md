`for` walks through a **range** of numbers for you. No counter to manage:

```rust
for i in 1..=3 {
    println!("{i}");   // prints 1, then 2, then 3
}
```

- `1..=3` means 1, 2, 3 (the `=` includes the last number).
- `1..3` means 1, 2 (it stops before 3).

**Your task:** `factorial(n)` multiplies all the numbers from 1 to `n`.
`factorial(4)` must give `24`  (1 × 2 × 3 × 4).
And `factorial(0)` is `1`.
