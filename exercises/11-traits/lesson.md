A **trait** describes behavior that several types can share (like an interface):

```rust
trait Describe {
    fn describe(&self) -> String;              // each type writes its own
    fn shout(&self) -> String {                // default method, can be reused
        self.describe().to_uppercase()
    }
}

impl Describe for Point {
    fn describe(&self) -> String { format!("({}, {})", self.x, self.y) }
}
```

`&dyn Describe` or `impl Describe` lets a function accept **any** type with that trait.

**Your task:**

1. Implement `Speak` for `Dog` (`"Rex says Woof"`) and for `Cat` (`"Tom says Meow"`).
2. Write the default method `speak_loudly`: the `speak()` text in UPPERCASE.
