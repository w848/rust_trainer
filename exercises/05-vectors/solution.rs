fn evens(numbers: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for &n in numbers {
        if n % 2 == 0 {
            result.push(n);
        }
    }
    result
}

// With iterators (see exercise 12):
// numbers.iter().copied().filter(|n| n % 2 == 0).collect()

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}", evens(&numbers));
}
