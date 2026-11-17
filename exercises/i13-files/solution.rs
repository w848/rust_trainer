use std::fs;
use std::io;

fn count_lines(path: &str) -> io::Result<usize> {
    Ok(fs::read_to_string(path)?.lines().count())
}

fn longest_line(path: &str) -> io::Result<Option<String>> {
    let text = fs::read_to_string(path)?;
    Ok(text.lines().max_by_key(|line| line.len()).map(String::from))
}

fn main() {
    fs::write("notes.txt", "buy milk\nlearn Rust traits and lifetimes\ncall mum\n").unwrap();
    println!("{:?}", count_lines("notes.txt"));
    println!("{:?}", longest_line("notes.txt"));
    println!("{:?}", count_lines("missing.txt"));
}
