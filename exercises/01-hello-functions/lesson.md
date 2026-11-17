In Rust, a function declares the **type** of each parameter and of its return value:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

The **last expression** without a `;` is the return value. No `return` needed.

Text comes in two flavors:

- `&str`: a borrowed piece of text, like `"Ferris"`.
- `String`: text you own and can grow. `format!("...")` builds one.

**Your task:** write `greet(name: &str) -> String` that returns `"Hello, NAME!"`.
`greet("Ferris")` must give `"Hello, Ferris!"`.
