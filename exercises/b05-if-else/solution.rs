fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("bigger of 3 and 8: {}", bigger(3, 8));
}
