#[derive(Debug, Clone, PartialEq)]
struct Task {
    id: u32,
    title: String,
    done: bool,
}

struct TodoList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TodoList {
    fn new() -> TodoList {
        todo!()
    }

    fn add(&mut self, title: &str) -> u32 {
        todo!()
    }

    fn complete(&mut self, id: u32) -> Result<(), String> {
        todo!()
    }

    fn remove(&mut self, id: u32) -> Option<Task> {
        todo!()
    }

    fn pending(&self) -> Vec<&str> {
        todo!()
    }

    fn summary(&self) -> String {
        todo!()
    }
}

fn main() {
    let mut list = TodoList::new();
    let milk = list.add("buy milk");
    list.add("learn Rust");
    list.add("call mum");
    list.complete(milk).unwrap();
    println!("to do: {:?}", list.pending());
    println!("{}", list.summary());
    println!("{:?}", list.complete(42));
}
