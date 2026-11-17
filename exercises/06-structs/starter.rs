struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        todo!()
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        todo!()
    }
}

fn main() {
    let big = Rectangle { width: 30, height: 50 };
    let small = Rectangle { width: 10, height: 40 };
    println!("big area: {}", big.area());
    println!("big can hold small: {}", big.can_hold(&small));
}
