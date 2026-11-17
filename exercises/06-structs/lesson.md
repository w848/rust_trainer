A `struct` groups named fields. An `impl` block gives it **methods**:

```rust
struct Point { x: f64, y: f64 }

impl Point {
    fn new(x: f64, y: f64) -> Point { Point { x, y } }  // "associated function"
    fn norm(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
}

let p = Point::new(3.0, 4.0);
println!("{}", p.norm()); // 5
```

`&self` means the method **borrows** the value it's called on.

**Your task:** complete `Rectangle`:

- `area(&self) -> u32`: width × height
- `can_hold(&self, other: &Rectangle) -> bool`: true if `other` fits strictly inside
  (`self` is wider **and** taller than `other`)
