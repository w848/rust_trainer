`{:?}` prints with `Debug` (for programmers). To print a type with plain `{}`,
implement the standard trait `std::fmt::Display`:

```rust
use std::fmt;

struct Euro(u32);

impl fmt::Display for Euro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} €", self.0)
    }
}
// println!("{}", Euro(5)) prints "5 €", and Euro(5).to_string() gives "5 €"
```

`write!` works like `format!` but writes into `f`.
`{:02}` pads a number with zeros to 2 digits: `5` → `05`.

**Your tasks:**

1. Implement `Display` for `Time` so it prints like `09:05`.
2. `add_minutes(&self, extra)` gives a **new** `Time`, going around midnight:
   `23:50` + 20 minutes = `00:10`.
