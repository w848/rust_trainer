fn sum_to(n: u32) -> u32 {
    let mut sum = 0;
    let mut i = 1;
    while i <= n {
        sum += i;
        i += 1;
    }
    sum
}

fn main() {
    println!("1 + 2 + 3 + 4 = {}", sum_to(4));
}
