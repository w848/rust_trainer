Normally a value has **one** owner. Sometimes several parts of a program need the same value,
and need to change it. Two tools work together:

- `Rc<T>` (reference counted): several owners. `Rc::clone(&x)` adds an owner, no copy.
- `RefCell<T>`: lets you change the inside even through a shared reference.
  `.borrow()` to read, `.borrow_mut()` to change.

```rust
use std::cell::RefCell;
use std::rc::Rc;

let score = Rc::new(RefCell::new(0));
let same_score = Rc::clone(&score);
*same_score.borrow_mut() += 10;
println!("{}", score.borrow());          // 10: it's the same value
println!("{}", Rc::strong_count(&score)); // 2 owners
```

`RefCell` still checks the borrowing rules, but **while the program runs**:
two `borrow_mut()` at the same time = a crash ("already borrowed").

**Your tasks:** `open_account`, `deposit`, `balance`, and `withdraw`, which returns
`Err("not enough money".to_string())` and changes nothing if the balance is too low.
