mod models;

use crate::models::Player;
use crate::models::Position;

fn main() {
    let player = Player {
        id: 1,
        name: String::from("Anthony Volpe"),
        positions: vec![Position::SS],
    };

    println!("Name: {}, ID: {}", player.name, player.id);
    for position in player.positions.iter() {
        println!("Plays: {}", position.name());
    }
}
