struct Item {
    name: String,
    price: f64,
    quantity: u32,
}

fn item(name: &str, price: f64, quantity: u32) -> Item {
    Item { name: name.to_string(), price, quantity }
}

fn total_value(items: &[Item]) -> f64 {
    items.iter().map(|i| i.price * i.quantity as f64).sum()
}

fn most_stocked(items: &[Item]) -> Option<&str> {
    items.iter().max_by_key(|i| i.quantity).map(|i| i.name.as_str())
}

fn main() {
    let shop = vec![item("apple", 0.5, 40), item("bread", 2.0, 5), item("milk", 1.25, 12)];
    println!("total value: {}", total_value(&shop));
    println!("most stocked: {:?}", most_stocked(&shop));
}
