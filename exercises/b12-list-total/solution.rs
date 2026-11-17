fn total(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for n in numbers {
        sum += n;
    }
    sum
}

fn main() {
    let my_numbers = [4, 8, 15, 16, 23, 42];
    println!("total: {}", total(&my_numbers));
}
