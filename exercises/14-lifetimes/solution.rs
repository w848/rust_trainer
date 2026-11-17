fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if b.len() > a.len() {
        b
    } else {
        a
    }
}

fn main() {
    let first = String::from("a long sentence");
    let second = "short";
    println!("{}", longest(&first, second));
}
