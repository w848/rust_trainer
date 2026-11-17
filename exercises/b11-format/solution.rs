fn introduce(name: &str, age: u32) -> String {
    format!("My name is {name} and I am {age}.")
}

fn main() {
    println!("{}", introduce("Mat", 30));
}
