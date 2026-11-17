Enum variants can carry named fields, and an `enum` can have an `impl` like a struct:

```rust
enum Pet {
    Dog { name: String },
    Fish,
}

impl Pet {
    fn sound(&self) -> &str {
        match self {
            Pet::Dog { name } => "woof",
            Pet::Fish => "...",
        }
    }
}
```

In a `match` you can take the fields out by writing their names: `Pet::Dog { name }`.

**Your tasks:**

1. `area(&self)`: circle = π × r², rectangle = width × height, triangle = base × height / 2.
2. `name(&self)`: `"circle"`, `"rectangle"` or `"triangle"`.
3. `total_area(shapes)`: the sum of all the areas.
