use std::f64::consts::PI;

enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => base * height / 2.0,
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle { .. } => "circle",
            Shape::Rectangle { .. } => "rectangle",
            Shape::Triangle { .. } => "triangle",
        }
    }
}

fn total_area(shapes: &[Shape]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn main() {
    let shapes = vec![
        Shape::Circle { radius: 1.0 },
        Shape::Rectangle { width: 2.0, height: 3.0 },
        Shape::Triangle { base: 4.0, height: 3.0 },
    ];
    for shape in &shapes {
        println!("{}: {:.2}", shape.name(), shape.area());
    }
    println!("total: {:.2}", total_area(&shapes));
}
