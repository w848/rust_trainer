fn sum_of_odd_squares(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|&&n| n % 2 != 0)
        .map(|&n| n * n)
        .sum()
}

fn main() {
    println!("{}", sum_of_odd_squares(&[1, 2, 3, 4, 5]));
}
