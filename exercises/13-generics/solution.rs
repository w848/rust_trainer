fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut largest = items[0];
    for &item in items {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("{}", largest(&[34, 50, 25, 100, 65]));
    println!("{}", largest(&['y', 'm', 'a', 'q']));
}
