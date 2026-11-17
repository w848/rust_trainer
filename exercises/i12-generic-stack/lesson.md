A struct can be generic too. Write `impl<T>` to give it methods for any `T`:

```rust
struct Pair<T> {
    a: T,
    b: T,
}

impl<T> Pair<T> {
    fn first(&self) -> &T {
        &self.a
    }
}
```

A **stack** is a pile: you `push` on top and `pop` from the top (last in, first out).
A `Vec` already does that, so your stack just wraps one.
Useful: `vec.pop()` gives `Option<T>`, `vec.last()` gives `Option<&T>`.

**Your tasks:**

1. Write the `Stack<T>` methods.
2. `balanced(text)` uses a `Stack<char>` to check that brackets `()`, `[]`, `{}` are
   correctly opened and closed. `"{[()]}"` is balanced, `"([)]"` and `"(("` are not.
   Other characters are ignored.
