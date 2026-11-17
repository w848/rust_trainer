An `enum` is a type that is **one of several variants**. Variants can carry data:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
```

`match` handles each variant, and the compiler checks you didn't forget one:

```rust
match msg {
    Message::Quit => println!("bye"),
    Message::Move { x, y } => println!("to {x},{y}"),
    Message::Write(text) => println!("{text}"),
}
```

**Your task:** write the `area(&self) -> f64` method of `Shape`:

- `Circle(radius)`: π × radius²
- `Square(side)`: side²
- `Rectangle(width, height)`: width × height
