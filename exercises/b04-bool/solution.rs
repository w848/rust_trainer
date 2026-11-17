fn is_adult(age: u32) -> bool {
    age >= 18
}

fn main() {
    println!("Is 20 adult? {}", is_adult(20));
}
