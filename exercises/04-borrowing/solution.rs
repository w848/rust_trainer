fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// By hand, with byte positions:
// let s = s.trim_start();
// match s.find(' ') {
//     Some(end) => &s[..end],
//     None => s,
// }

fn main() {
    let sentence = String::from("hello wonderful world");
    let word = first_word(&sentence);
    println!("first word: {word}");
}
