`let` gives a name to a value, so you can use it later:

```rust
fn price_with_tax(price: i32) -> i32 {
    let tax = price / 5;   // a line with let ends with ;
    price + tax            // the last line has no ;
}
```

Lines that **do** something end with `;`.
The last line, the value you give back, has **no** `;`.

**Your task:** `hours_to_minutes(hours)` gives back the number of minutes.
Use a `let` to store the result first, then give it back.
`hours_to_minutes(2)` must give `120`.
