struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        todo!()
    }

    fn push(&mut self, item: T) {
        todo!()
    }

    fn pop(&mut self) -> Option<T> {
        todo!()
    }

    fn peek(&self) -> Option<&T> {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn is_empty(&self) -> bool {
        todo!()
    }
}

fn balanced(text: &str) -> bool {
    let mut stack: Stack<char> = Stack::new();
    todo!()
}

fn main() {
    for text in ["{[()]}", "([)]", "fn main() { println!(\"hi\"); }"] {
        println!("{text}  ->  {}", balanced(text));
    }
}
