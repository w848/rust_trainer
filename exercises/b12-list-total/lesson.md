A list of values is written with `[ ]`:

```rust
let scores = [10, 20, 30];
println!("{}", scores[0]);      // 10: the first one is number 0
println!("{}", scores.len());   // 3
```

`for` can walk through a list too:

```rust
for s in scores {
    println!("{s}");
}
```

In this exercise the function receives `&[i32]`: "a list of `i32` that I can read".

**Your task:** `total(numbers)` adds all the numbers of the list.
`total(&[1, 2, 3])` must give `6`.
