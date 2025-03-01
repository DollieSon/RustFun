use core::fmt;
use std::{cmp::Ordering, fmt::Debug};

use super::entities::Entity;
pub struct Move<'a> {
    name: String,
    cost: u128,
    damage: u128,
    speed: i128,
    curr_speed: i128,
    effect: fn(&mut Entity, &mut Move),
    owner: Option<&'a mut Entity>,
}

impl<'owner> Move<'owner> {
    pub fn new(
        name: &str,
        cost: u128,
        damage: u128,
        speed: i128,
        effect: fn(&mut Entity, &mut Move),
    ) -> Self {
        Move {
            name: name.to_string(),
            cost: cost,
            damage: damage,
            speed: speed,
            curr_speed: speed,
            effect: effect,
            owner: None,
        }
    }
    pub fn speed_sort<'a, 'b>(a: &'a Move, b: &'b Move) -> Ordering {
        a.curr_speed.cmp(&b.curr_speed)
    }
    pub fn set_owner(mut self, owner: &'owner mut Entity) {
        self.owner = Some(owner);
    }
    pub fn apply_effect(mut self, target: &mut Entity) {
        //check if entity has cost
        (self.effect)(target, &mut self);
        self.curr_speed += self.speed;
    }
}

impl Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Move")
            .field("Name", &self.name)
            .field("Speed", &self.curr_speed)
            .field("Damage", &self.damage)
            .field("Cost", &self.cost)
            .finish()
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}\t\n",
            self.name, self.cost, self.damage, self.curr_speed
        )
    }
}

impl Clone for Move {
    fn clone(&self) -> Self {
        Move {
            name: self.name.clone(),
            cost: self.cost,
            damage: self.damage,
            curr_speed: self.curr_speed,
            // effect: self.effect.clone(),
        }
    }
}
