Time to mix things you already know: a `struct`, a list of them, and iterators.

```rust
let prices = vec![2.0, 3.5];
let total: f64 = prices.iter().sum();
let biggest = vec![3, 9, 4].into_iter().max();   // Some(9)
```

Two useful tools:

- `as f64` converts a number to another type: `quantity as f64`.
- `max_by_key(|x| ...)` finds the element with the biggest *key*. It gives an `Option`
  (the list may be empty).

**Your tasks:**

1. `total_value(items)`: the sum of `price × quantity` for every item.
2. `most_stocked(items)`: the **name** of the item with the biggest quantity,
   or `None` if the list is empty.
