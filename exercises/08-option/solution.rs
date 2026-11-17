fn find_index(numbers: &[i32], target: i32) -> Option<usize> {
    for (index, &value) in numbers.iter().enumerate() {
        if value == target {
            return Some(index);
        }
    }
    None
}

// The standard library already has it:
// numbers.iter().position(|&n| n == target)

fn main() {
    let numbers = [4, 8, 15, 16, 23, 42];
    match find_index(&numbers, 15) {
        Some(i) => println!("15 is at position {i}"),
        None => println!("15 is not there"),
    }
}
