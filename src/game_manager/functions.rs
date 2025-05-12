use crate::player;
use rand::{Rng};

pub fn spawn_enemy() -> player::Character {
    let mut generator = rand::rng();  
    let names = ["goblin", "wolf", "bat", "rat", "witch", "skeleton", "zombie", "human"];
    let name = names[generator.random_range(0..names.len())];

    match name {
        "bat" => {
            
            player::Character {
                name: String::from("bat"),
                hp: 8,
                max_hp: 8,
                damage: 3,
            }
        }
        "goblin" => player::Character {
            name: String::from("goblin"),
            hp: 30,
            max_hp: 30,
            damage: 5,
        },
        "wolf" => player::Character {
            name: String::from("wolf"),
            hp: 20,
            max_hp: 20,
            damage: 7,
        },
        "rat" => player::Character {
            name: String::from("rat"),
            hp: 10,
            max_hp: 10,
            damage: 3,
        },
        "witch" => player::Character {
            name: String::from("witch"),
            hp: 40,
            max_hp: 40,
            damage: 10,
        },
        "skeleton" => player::Character {
            name: String::from("skeleton"),
            hp: 25,
            max_hp: 25,
            damage: 6,
        },
        "zombie" => player::Character {
            name: String::from("zombie"),
            hp: 35,
            max_hp: 35,
            damage: 8,
        },
        "human" => player::Character {
            name: String::from("human"),
            hp: 20,
            max_hp: 20,
            damage: 4,
        },
        _ => player::Character {
            name: String::from("unknown"),
            hp: 15,
            max_hp: 15,
            damage: 5,
        },
    }
}