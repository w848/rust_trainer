#[derive(Debug, PartialEq)]
enum ParseError {
    Empty,
    MissingEquals,
    BadNumber(String),
}

fn parse_setting(line: &str) -> Result<(String, i32), ParseError> {
    todo!()
}

fn parse_all(text: &str) -> Result<Vec<(String, i32)>, ParseError> {
    todo!()
}

fn main() {
    println!("{:?}", parse_setting("volume = 42"));
    println!("{:?}", parse_setting("volume 42"));
    println!("{:?}", parse_all("width=800\n\nheight=600"));
}
