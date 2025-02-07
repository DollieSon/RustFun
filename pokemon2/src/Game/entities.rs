use super::moves::Move;

pub struct Entity {
    name: String,
    hp: u128,
    speed: u128,
    damage: u128,
    mana: u128,
    moves: Vec<Move>,
}

impl Entity {
    pub fn new(name: String, stat: (u128, u128, u128, u128), moves: [Move; 4]) -> Self {
        return Entity {
            name: name,
            hp: stat.0,
            speed: stat.1,
            damage: stat.2,
            mana: stat.3,
            moves: Vec::from(moves),
        };
    }
}
