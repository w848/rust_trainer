struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

fn balanced(text: &str) -> bool {
    let mut stack: Stack<char> = Stack::new();
    for c in text.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                let opening = match c {
                    ')' => '(',
                    ']' => '[',
                    _ => '{',
                };
                if stack.pop() != Some(opening) {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

fn main() {
    for text in ["{[()]}", "([)]", "fn main() { println!(\"hi\"); }"] {
        println!("{text}  ->  {}", balanced(text));
    }
}
