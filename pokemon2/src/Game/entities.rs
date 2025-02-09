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
    pub fn new(name: String, stat: (u128, u128, u128, u128), mut moves: Vec<Move>) -> Self {
        moves.sort_by(Move::speed_sort);
        return Entity {
            name: name,
            hp: stat.0,
            speed: stat.1,
            damage: stat.2,
            mana: stat.3,
            moves: moves,
        };
    }
}

impl Clone for Entity {
    fn clone(&self) -> Self {
        Entity::new(
            self.name.clone(),
            (self.hp, self.speed, self.damage, self.mana),
            self.moves.clone(),
        )
    }
}
