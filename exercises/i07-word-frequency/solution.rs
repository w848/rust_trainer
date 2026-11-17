use std::collections::HashMap;

fn word_counts(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let word = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
        if !word.is_empty() {
            *counts.entry(word).or_insert(0) += 1;
        }
    }
    counts
}

fn top_words(text: &str, n: usize) -> Vec<(String, usize)> {
    let mut pairs: Vec<(String, usize)> = word_counts(text).into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    pairs.truncate(n);
    pairs
}

fn main() {
    let text = "The cat sat. The cat ran! A dog? The end.";
    println!("{:?}", top_words(text, 3));
}
