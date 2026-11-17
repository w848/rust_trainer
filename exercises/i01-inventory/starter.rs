struct Item {
    name: String,
    price: f64,
    quantity: u32,
}

fn item(name: &str, price: f64, quantity: u32) -> Item {
    Item { name: name.to_string(), price, quantity }
}

fn total_value(items: &[Item]) -> f64 {
    todo!()
}

fn most_stocked(items: &[Item]) -> Option<&str> {
    todo!()
}

fn main() {
    let shop = vec![item("apple", 0.5, 40), item("bread", 2.0, 5), item("milk", 1.25, 12)];
    println!("total value: {}", total_value(&shop));
    println!("most stocked: {:?}", most_stocked(&shop));
}
