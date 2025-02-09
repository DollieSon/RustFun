use core::fmt;
use std::fmt::write;

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
    pub fn pop_move(&mut self) -> Option<Move> {
        self.moves.pop()
    }
    //Should Return a result ,but be lazy
    pub fn insert_move(&mut self, move_given: Move) {
        self.moves.insert(0, move_given);
        self.moves.sort_by(Move::speed_sort);
    }
    pub fn get_moves(&self) -> &Vec<Move> {
        &self.moves
    }
}
impl fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}Stats:\n\tHealth: {}\tSpeed: {}\tDamage: {}\tMana: {}\nMoves:\n\t{:?}",
            self.name, self.hp, self.speed, self.damage, self.mana, self.moves,
        )
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
