fn final_score(score: i32) -> i32 {
    let mut total = score;
    total += 10;
    total *= 2;
    total
}

fn main() {
    println!("final score: {}", final_score(5));
}
