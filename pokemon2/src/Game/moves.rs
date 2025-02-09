use std::cmp::Ordering;

use super::entities::Entity;
pub struct Move {
    name: String,
    cost: u128,
    damage: u128,
    curr_speed: u128,
    effect: fn(&mut Entity, &mut Move),
}

impl Move {
    pub fn speed_sort<'a, 'b>(a: &'a Move, b: &'b Move) -> Ordering {
        a.curr_speed.cmp(&b.curr_speed)
    }
}

impl Clone for Move {
    fn clone(&self) -> Self {
        Move {
            name: self.name.clone(),
            cost: self.cost,
            damage: self.damage,
            curr_speed: self.curr_speed,
            effect: self.effect.clone(),
        }
    }
}
