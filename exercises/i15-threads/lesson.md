`thread::spawn` runs a closure **at the same time** as the rest of the program.
It gives a handle; `.join()` waits for the thread and gets its result:

```rust
use std::thread;

let data = vec![1, 2, 3];
let handle = thread::spawn(move || data.iter().sum::<i32>());
let total = handle.join().unwrap();   // 6
```

`move` gives the closure **ownership** of what it uses (here `data`), because the thread
may live longer than the function that created it.

**Channels** send values from one thread to another:

```rust
use std::sync::mpsc;

let (sender, receiver) = mpsc::channel();
thread::spawn(move || sender.send(42).unwrap());
println!("{}", receiver.recv().unwrap());   // 42
```

`receiver.iter()` receives values until every sender is gone (the thread ended).

**Your tasks:**

1. `parallel_sum(numbers, workers)`: split the list into `workers` parts
   (`numbers.chunks(size)`), sum each part in its own thread, then add the results.
2. `squares(n)`: a thread sends `1², 2², ... n²` through a channel; collect them.
