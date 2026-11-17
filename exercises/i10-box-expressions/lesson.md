An `enum` can't contain itself directly: its size would be infinite.
`Box<T>` puts a value on the **heap** and only stores a pointer, which has a fixed size:

```rust
enum List {
    Item(i32, Box<List>),
    End,
}
```

That's how you build **trees**. Here, `(1 + 2) * 3` is:

```text
      Mul
     /   \
   Add    3
  /   \
 1     2
```

To work on a tree, use a **recursive** function: it calls itself on the branches.
`eval(a)` works even if `a` is a `&Box<Expr>`: Rust looks through the box for you.

**Your tasks:**

1. `eval(e)`: compute the value.
2. `to_text(e)`: `Add` and `Mul` are written `(a + b)` and `(a * b)`, `Neg` is `-a`.
   `mul(add(num(1), num(2)), num(3))` → `"((1 + 2) * 3)"`.
