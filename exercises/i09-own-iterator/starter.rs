struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        todo!()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        todo!()
    }
}

fn even_sum_below(limit: u64) -> u64 {
    todo!()
}

fn main() {
    let first: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("{first:?}");
    println!("{}", even_sum_below(100));
}
