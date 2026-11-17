trait Speak {
    fn speak(&self) -> String;

    fn speak_loudly(&self) -> String {
        todo!()
    }
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

// Write: impl Speak for Dog { ... }

// Write: impl Speak for Cat { ... }

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
