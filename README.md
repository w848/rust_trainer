# 🦀 Rust Trainer

Practise Rust in your browser. Each exercise has a short lesson, a code editor
and a terminal. Your code is compiled by the **real Rust compiler** on your PC
and checked by tests, so you see real compiler errors and real test results.

## Start it

Double-click **`start.cmd`**, or in a terminal:

```powershell
cd H:\Softwares\rust_trainer
cargo run --release
```

The browser opens at http://localhost:8090. Stop it with **Enter** in its window, or just close the window.
The first start takes ~20 s (it builds the server); after that it's instant.

## How to use it

1. Pick an exercise on the left and read the lesson.
2. Write your code in the editor (replace the `todo!()`).
3. **✔ Check** (Ctrl+Enter): compiles your code with the exercise's tests and shows
   ✔ / ✘ for each test. When they all pass, the exercise is marked solved.
4. **▶ Run** (Ctrl+Shift+Enter): compiles and runs your `main()` to see what it prints.
5. Stuck? **💡 Hint**. Done? **👁 Solution** shows a reference solution next to your code.

Your code and progress are saved in the browser automatically.
**↺ Reset** puts the starting code back.

## The exercises

### Beginner (start here)

| # | Topic |
|---|-------|
| 1 | Your first function (give back a number) |
| 2 | Parameters (`add(a, b)`) |
| 3 | Variables with `let` |
| 4 | `bool` and comparisons |
| 5 | `if` / `else` |
| 6 | `else if` |
| 7 | `let mut` |
| 8 | `while` loops |
| 9 | `for` loops and ranges |
| 10 | Text: `&str` and `String` |
| 11 | `format!` |
| 12 | Lists (arrays) and `for` |
| 13 | Letters (`char`) with `if` inside a loop |

### Practice

| # | Topic |
|---|-------|
| 1 | Functions & `String` |
| 2 | `mut` & `for` loops |
| 3 | `if` / `else` (FizzBuzz) |
| 4 | Ownership & borrowing (`&str`) |
| 5 | `Vec` & slices |
| 6 | `struct` & `impl` |
| 7 | `enum` & `match` |
| 8 | `Option<T>` |
| 9 | `Result<T, E>` and `?` |
| 10 | `HashMap` & the entry API |
| 11 | Traits |
| 12 | Iterators & closures |
| 13 | Generics & trait bounds |
| 14 | Lifetimes |

### Intermediate (after Practice)

| # | Topic |
|---|-------|
| 1 | A shop inventory: `struct` + `Vec` + iterators |
| 2 | Shapes: `enum` with data + `impl` |
| 3 | Title Case: `chars`, `split_whitespace`, `join` |
| 4 | Your own error type, `ok_or`, `map_err`, collecting `Result`s |
| 5 | The `Display` trait |
| 6 | Leaderboard: `sort_by` + `then` |
| 7 | Most used words: `HashMap` + sorting |
| 8 | Iterator toolbox: `filter_map`, `zip`, `enumerate` |
| 9 | Implementing `Iterator` (Fibonacci) |
| 10 | A tiny calculator: `Box` & recursive enums |
| 11 | Modules: `mod`, `pub`, `super` |
| 12 | A generic `Stack<T>` (bracket checker) |
| 13 | Reading files: `std::fs` + `io::Result` |
| 14 | A shared bank account: `Rc<RefCell<T>>` |
| 15 | Threads & channels |
| 16 | Mini project: a to-do list |

## Add your own exercise

Create a folder in `exercises/`, for example `15-my-topic/`, with 5 files:

| File | Content |
|------|---------|
| `meta.json` | `{ "title": "...", "topic": "...", "hint": "..." }`, add `"level": "Beginner"` or `"Intermediate"` to choose its group (default: Practice) |
| `lesson.md` | the lesson (Markdown: `code`, **bold**, lists, code blocks) |
| `starter.rs` | the code you start from (use `todo!()` for the parts to write) |
| `solution.rs` | a working solution |
| `tests.rs` | `#[test]` functions; they can use everything from your code |

Folders are listed in alphabetical order, so number them. Reload the page to see it.

## How it works

- `src/main.rs` is a small web server (tiny_http + serde_json). For **Check**, it
  puts your code and the exercise's tests in one file, runs
  `rustc --test`, runs the result, and reads the `test ... ok` lines.
  For **Run**, it compiles normally and runs `main()`.
- Programs are stopped after 5 seconds, so an endless loop can't hang anything.
- The server only listens on `127.0.0.1`: nobody else on the network can use it.
- `static/` holds the page (`index.html`, `style.css`, `app.js`).
