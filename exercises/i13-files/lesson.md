Reading a file can fail (missing file, no permission...), so it returns a `Result`:

```rust
use std::fs;
use std::io;

fn first_line(path: &str) -> io::Result<String> {
    let text = fs::read_to_string(path)?;   // ? returns the error to the caller
    Ok(text.lines().next().unwrap_or("").to_string())
}
```

- `io::Result<T>` is short for `Result<T, io::Error>`.
- `text.lines()` iterates over the lines (without the `\n`).
- `fs::write(path, text)` writes a file.

**Your tasks:**

1. `count_lines(path)`: the number of lines in the file.
2. `longest_line(path)`: the longest line, or `None` for an empty file.

Both must return an error (not crash!) when the file doesn't exist.
Press ▶ Run: `main` creates a small file and reads it back.
