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
        TodoList { tasks: Vec::new(), next_id: 1 }
    }

    fn add(&mut self, title: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.push(Task { id, title: title.to_string(), done: false });
        id
    }

    fn complete(&mut self, id: u32) -> Result<(), String> {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => {
                task.done = true;
                Ok(())
            }
            None => Err(format!("no task with id {id}")),
        }
    }

    fn remove(&mut self, id: u32) -> Option<Task> {
        let index = self.tasks.iter().position(|t| t.id == id)?;
        Some(self.tasks.remove(index))
    }

    fn pending(&self) -> Vec<&str> {
        self.tasks.iter().filter(|t| !t.done).map(|t| t.title.as_str()).collect()
    }

    fn summary(&self) -> String {
        let done = self.tasks.iter().filter(|t| t.done).count();
        format!("{} done, {} to do", done, self.tasks.len() - done)
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
