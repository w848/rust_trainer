trait Speak {
    fn speak(&self) -> String;

    fn speak_loudly(&self) -> String {
        self.speak().to_uppercase()
    }
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Speak for Dog {
    fn speak(&self) -> String {
        format!("{} says Woof", self.name)
    }
}

impl Speak for Cat {
    fn speak(&self) -> String {
        format!("{} says Meow", self.name)
    }
}

fn main() {
    let animals: Vec<Box<dyn Speak>> = vec![
        Box::new(Dog { name: "Rex".into() }),
        Box::new(Cat { name: "Tom".into() }),
    ];
    for animal in &animals {
        println!("{}", animal.speak());
        println!("{}", animal.speak_loudly());
    }
}
