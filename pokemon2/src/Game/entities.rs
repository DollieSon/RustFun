pub mod moves;
use moves::Move;
enum Entity {
    Base {
        name: String,
        base_stat: Stat,
        moves: Vec<Move>,
    },
    Fighting {
        original: Entity,
        current_stat: Stat,
        patttern: Vec<&Move>,
    },
}

struct Stat {
    hp: i32,
    speed: i32,
    damage: i32,
    mana: i32,
}

impl Entity::Base {
    fn to_fight(&self, fight: Vec<&Move>) -> Entity::Fighting {
        Fighting {
            original: &self,
            current_stat: Stat {
                hp: self.base_stat.hp,
                speed: self.base_stat.speed,
                damage: self.base_stat.damage,
                mana: self.base_stat.mana,
            },
            pattern: fight,
        }
    }
}
