fn sum_to(n: u32) -> u32 {
    let mut total = 0;
    for i in 1..=n {
        total += i;
    }
    total
}

// With an iterator, it fits on one line: (1..=n).sum()

fn main() {
    println!("sum_to(4) = {}", sum_to(4));
}
