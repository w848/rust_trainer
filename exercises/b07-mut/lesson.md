By default a variable **can't change**. Add `mut` (mutable) to allow it:

```rust
let mut lives = 3;
lives -= 1;       // same as: lives = lives - 1;
lives += 5;       // same as: lives = lives + 5;
```

Without `mut`, the compiler says `cannot assign twice to immutable variable`.
Try it to see the message!

**Your task:** in `final_score(score)`:

1. start a mutable variable at `score` (already done),
2. add `10` to it,
3. then multiply it by `2` (`*=`),
4. give it back.

`final_score(5)` must give `30`  (5 + 10 = 15, then 15 × 2 = 30).
