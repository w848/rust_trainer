use std::fs;
use std::io;

fn count_lines(path: &str) -> io::Result<usize> {
    todo!()
}

fn longest_line(path: &str) -> io::Result<Option<String>> {
    todo!()
}

fn main() {
    fs::write("notes.txt", "buy milk\nlearn Rust traits and lifetimes\ncall mum\n").unwrap();
    println!("{:?}", count_lines("notes.txt"));
    println!("{:?}", longest_line("notes.txt"));
    println!("{:?}", count_lines("missing.txt"));
}
