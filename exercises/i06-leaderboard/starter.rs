#[derive(Debug, Clone, PartialEq)]
struct Player {
    name: String,
    score: u32,
}

fn player(name: &str, score: u32) -> Player {
    Player { name: name.to_string(), score }
}

fn rank(mut players: Vec<Player>) -> Vec<Player> {
    todo!()
}

fn podium(players: Vec<Player>) -> Vec<String> {
    todo!()
}

fn main() {
    let players = vec![player("Zoe", 50), player("Mat", 80), player("Ana", 50), player("Bob", 10)];
    for (i, p) in rank(players.clone()).iter().enumerate() {
        println!("{}. {} ({})", i + 1, p.name, p.score);
    }
    println!("podium: {:?}", podium(players));
}
