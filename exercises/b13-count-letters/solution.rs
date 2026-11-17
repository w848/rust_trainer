fn count_letter(text: &str, letter: char) -> u32 {
    let mut count = 0;
    for c in text.chars() {
        if c == letter {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("a in banana: {}", count_letter("banana", 'a'));
}
