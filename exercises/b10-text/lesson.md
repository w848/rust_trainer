Text in Rust:

- `"hello"` written in your code is a `&str`: text you can **read**.
- A `String` is text you **own**: you can build it, change it, give it back.

Text has handy methods. You call them with a dot:

```rust
let name = "ferris";
let loud = name.to_uppercase();   // "FERRIS", a new String
let size = name.len();            // 6
```

**Your task:** `shout(word)` gives back the word in capital letters.
`shout("hello")` must give `"HELLO"`.
