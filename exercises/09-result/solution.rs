fn parse_age(s: &str) -> Result<u8, String> {
    let age: u8 = s.trim().parse().map_err(|_| "not a number".to_string())?;
    if age > 150 {
        return Err("too old".to_string());
    }
    Ok(age)
}

fn main() {
    for input in ["42", " 7 ", "abc", "200"] {
        match parse_age(input) {
            Ok(age) => println!("{input:?} -> age {age}"),
            Err(error) => println!("{input:?} -> error: {error}"),
        }
    }
}
