fn hours_to_minutes(hours: i32) -> i32 {
    let minutes = hours * 60;
    minutes
}

fn main() {
    println!("3 hours = {} minutes", hours_to_minutes(3));
}
