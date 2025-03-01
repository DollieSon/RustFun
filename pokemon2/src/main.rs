pub mod Game;
use Game::{entities::Entity, moves::*};

fn main() {
    let mut moves = Vec::<Move>::new();
    // moves.push(Move::new("Something", 21, 31, 0));
    let player = Entity::new("Brian".to_string(), (12, 11, 10, 9), moves);
    println!("{}", player);
}
