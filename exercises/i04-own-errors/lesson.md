Instead of returning a `String` as error, real programs define an **error enum**:
the caller can then `match` on exactly what went wrong.

```rust
#[derive(Debug, PartialEq)]
enum AgeError { Negative, TooOld }

fn check(age: i32) -> Result<i32, AgeError> {
    if age < 0 { return Err(AgeError::Negative); }
    Ok(age)
}
```

Tools to turn other things into *your* error:

- `option.ok_or(MyError::X)`: `Some(v)` → `Ok(v)`, `None` → `Err(MyError::X)`.
- `result.map_err(|_| MyError::Y)`: changes the error type.
- Then `?` returns early if it's an `Err`.
- `"a=b".split_once('=')` gives `Some(("a", "b"))`.

Bonus magic: an iterator of `Result`s can be collected into **one** `Result<Vec<_>, _>`.
It stops at the first error.

**Your tasks:**

1. `parse_setting("volume = 42")` → `Ok(("volume".to_string(), 42))` (spaces trimmed).
   Errors: empty line → `Empty`, no `=` → `MissingEquals`, bad number → `BadNumber(the text)`.
2. `parse_all(text)`: parse every non-empty line, or return the first error.
