`Vec<T>` is a growable list. `&[T]` is a **slice**: a borrowed view of a list.

```rust
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
let w = vec![10, 20, 30];   // the vec! macro
for x in &w { println!("{x}"); }
```

Taking `&[i32]` as a parameter accepts a `&Vec<i32>`, an array, or part of one.

**Your task:** write `evens(numbers: &[i32]) -> Vec<i32>` that returns only the
even numbers, in the same order. `evens(&[1, 2, 3, 4])` is `vec![2, 4]`.
Negative numbers count too: `-2` is even.
