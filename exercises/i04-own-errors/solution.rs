#[derive(Debug, PartialEq)]
enum ParseError {
    Empty,
    MissingEquals,
    BadNumber(String),
}

fn parse_setting(line: &str) -> Result<(String, i32), ParseError> {
    let line = line.trim();
    if line.is_empty() {
        return Err(ParseError::Empty);
    }
    let (key, value) = line.split_once('=').ok_or(ParseError::MissingEquals)?;
    let value = value.trim();
    let number = value
        .parse::<i32>()
        .map_err(|_| ParseError::BadNumber(value.to_string()))?;
    Ok((key.trim().to_string(), number))
}

fn parse_all(text: &str) -> Result<Vec<(String, i32)>, ParseError> {
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_setting)
        .collect()
}

fn main() {
    println!("{:?}", parse_setting("volume = 42"));
    println!("{:?}", parse_setting("volume 42"));
    println!("{:?}", parse_all("width=800\n\nheight=600"));
}
