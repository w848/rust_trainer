fn sign(n: i32) -> i32 {
    if n > 0 {
        1
    } else if n < 0 {
        -1
    } else {
        0
    }
}

fn main() {
    println!("{} {} {}", sign(7), sign(-3), sign(0));
}
