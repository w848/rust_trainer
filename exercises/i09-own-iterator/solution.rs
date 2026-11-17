struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { current: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let value = self.current;
        self.current = self.next;
        self.next += value;
        Some(value)
    }
}

fn even_sum_below(limit: u64) -> u64 {
    Fibonacci::new()
        .take_while(|&x| x < limit)
        .filter(|x| x % 2 == 0)
        .sum()
}

fn main() {
    let first: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("{first:?}");
    println!("{}", even_sum_below(100));
}
