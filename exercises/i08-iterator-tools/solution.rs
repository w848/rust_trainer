fn sum_valid(inputs: &[&str]) -> i32 {
    inputs.iter().filter_map(|s| s.trim().parse::<i32>().ok()).sum()
}

fn dot_product(a: &[i32], b: &[i32]) -> i32 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

fn positions_of(items: &[&str], target: &str) -> Vec<usize> {
    items
        .iter()
        .enumerate()
        .filter(|(_, item)| **item == target)
        .map(|(i, _)| i)
        .collect()
}

fn main() {
    println!("{}", sum_valid(&["4", " 6 ", "seven", "-2"]));
    println!("{}", dot_product(&[1, 2, 3], &[4, 5, 6]));
    println!("{:?}", positions_of(&["a", "b", "a", "c", "a"], "a"));
}
