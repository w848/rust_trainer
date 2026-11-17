**Generics** write one function for many types. `T` is a type parameter:

```rust
fn first<T>(items: &[T]) -> &T {
    &items[0]
}
```

To compare or copy `T`, you say which abilities it needs with **trait bounds**:

```rust
fn biggest<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

`PartialOrd` allows `>` and `<`; `Copy` allows copying the value out of the slice.

**Your task:** write `largest<T: PartialOrd + Copy>(items: &[T]) -> T` that
returns the largest item. It must work for numbers **and** for `char`.
You can assume the list is never empty.
