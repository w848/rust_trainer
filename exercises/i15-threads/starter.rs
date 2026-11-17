use std::sync::mpsc;
use std::thread;

fn parallel_sum(numbers: Vec<u64>, workers: usize) -> u64 {
    if numbers.is_empty() {
        return 0;
    }
    let size = (numbers.len() + workers - 1) / workers; // rounded up
    let mut handles: Vec<thread::JoinHandle<u64>> = Vec::new();
    // spawn one thread per chunk, push its handle

    todo!() // join all handles and add their results
}

fn squares(n: u64) -> Vec<u64> {
    let (sender, receiver) = mpsc::channel::<u64>();
    todo!()
}

fn main() {
    let numbers: Vec<u64> = (1..=1_000_000).collect();
    println!("sum with 4 threads: {}", parallel_sum(numbers, 4));
    println!("squares: {:?}", squares(5));
}
