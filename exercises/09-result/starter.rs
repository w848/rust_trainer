fn parse_age(s: &str) -> Result<u8, String> {
    todo!()
}

fn main() {
    for input in ["42", " 7 ", "abc", "200"] {
        match parse_age(input) {
            Ok(age) => println!("{input:?} -> age {age}"),
            Err(error) => println!("{input:?} -> error: {error}"),
        }
    }
}
