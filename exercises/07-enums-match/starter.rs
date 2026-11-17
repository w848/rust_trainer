use std::f64::consts::PI;

enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        todo!()
    }
}

fn main() {
    let shapes = [Shape::Circle(1.0), Shape::Square(2.0), Shape::Rectangle(2.0, 3.0)];
    for shape in &shapes {
        println!("{:.2}", shape.area());
    }
}
