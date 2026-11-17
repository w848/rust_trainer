`println!` **prints** text. `format!` builds the same text but **gives it back** as a `String`:

```rust
let city = "Paris";
println!("I live in {city}");          // prints it
let s = format!("I live in {city}");   // s is "I live in Paris"
```

Each `{name}` is replaced by the variable with that name.

**Your task:** `introduce(name, age)` gives back

`"My name is NAME and I am AGE."`

`introduce("Mat", 30)` must give `"My name is Mat and I am 30."`
(watch the spaces and the final dot!)
