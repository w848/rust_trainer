Big programs are split into **modules** with `mod`. Everything inside is **private**
by default: only code in that module can use it. `pub` makes it usable from outside.

```rust
mod kitchen {
    pub fn cook() -> &'static str { secret_recipe() }
    fn secret_recipe() -> &'static str { "pasta" }   // private
}

fn main() {
    kitchen::cook();            // OK
    // kitchen::secret_recipe() // error: function is private
}
```

- Use `module::name` to reach inside, `super::name` to reach the parent module.
- `use kitchen::cook;` lets you write just `cook()`.
- `&'static str` is text written in the program itself, which lives forever.

The first time you press Check here, you get **"function is private"** errors.
That's the point of this exercise: read them, then fix them!

**Your tasks:**

1. Make the functions and the `describe` module public, and write their bodies:
   °F = °C × 9 / 5 + 32, and the other way around.
2. `feeling(c)`: `"cold"` under 10 °C, `"nice"` under 25 °C, otherwise `"hot"`.
3. `feeling_f(f)`: the same, from °F (reuse `f_to_c` with `super::`).
