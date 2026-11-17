The final boss of this group: a small to-do list, like the inside of a real app.
It uses structs, `Vec`, `Option`, `Result`, iterators and `format!`.

New helpers you might want:

- `iter_mut()`: iterate with permission to change the elements.
- `find(|t| ...)`: the first element that matches, as an `Option`.
- `position(|t| ...)`: the **index** of the first match, as an `Option<usize>`.
- `vec.remove(index)`: takes the element out and gives it back.
- `?` also works on `Option` inside a function that returns `Option`.

**Your tasks:**

- `add(title)`: adds a task and returns its id. Ids start at 1 and are **never reused**
  (even after a remove).
- `complete(id)`: marks it done, or `Err("no task with id 7")`.
- `remove(id)`: takes it out and returns it (`None` if it doesn't exist).
- `pending()`: the titles not done yet, in order.
- `summary()`: like `"1 done, 2 to do"`.

When it passes, you could make it a real program: read commands from the keyboard
with `std::io::stdin().read_line(...)` and save the list to a file. 🚀
