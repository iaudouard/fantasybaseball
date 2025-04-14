enum PositionType {
    Pitcher,
    Batter,
}

enum Position {
    SS,
}

struct Player {
    id: u32,
    name: String,
    positions: Vec<Position>,
}

impl Position {
    fn position_type(&self) -> PositionType {
        match self {
            Position::SS => PositionType::Batter,
        }
    }
}

fn main() {
    println!("Hello world");
    let player = Player {
        id: 1,
        name: String::from("Anthony Volpe"),
        positions: vec![Position::SS],
    };

    println!("Name: {}, ID: {}", player.name, player.id);
}
