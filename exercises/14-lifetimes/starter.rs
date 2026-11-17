// The signature is incomplete on purpose: try "Check" to see the compiler's advice.
fn longest(a: &str, b: &str) -> &str {
    todo!()
}

fn main() {
    let first = String::from("a long sentence");
    let second = "short";
    println!("{}", longest(&first, second));
}
