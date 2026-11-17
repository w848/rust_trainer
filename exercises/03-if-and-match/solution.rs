fn fizzbuzz(n: u32) -> String {
    if n % 15 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else {
        n.to_string()
    }
}

// Another way, with match on a tuple:
// match (n % 3, n % 5) {
//     (0, 0) => "FizzBuzz".into(),
//     (0, _) => "Fizz".into(),
//     (_, 0) => "Buzz".into(),
//     _ => n.to_string(),
// }

fn main() {
    for n in 1..=15 {
        println!("{}", fizzbuzz(n));
    }
}
