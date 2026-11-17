fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
        None => String::new(),
    }
}

fn title_case(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .map(capitalize)
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    println!("{}", capitalize("rUST"));
    println!("{}", title_case("the rust programming language"));
}
