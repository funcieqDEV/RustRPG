mod functions;
use std::io;
use crate::player;

pub fn fight(usr: &mut player::Character){
    let mut enemy = functions::spawn_enemy();
    loop {
        let mut choice = String::new();
        println!("{} {}/{}hp",enemy.name,enemy.hp,enemy.max_hp);
        println!("{}, {}/{}hp damage: {}",usr.name,usr.hp,usr.max_hp,usr.damage);
        println!("#1. hit");
        println!("#2. escape");
        print!("> ");
        io::stdin().read_line(&mut choice).expect("...");

        match choice.trim() {
            "1" => {
                println!("{} deal {} hp!", enemy.name,enemy.damage);
                usr.hp -= enemy.damage as i8;
                println!("{} you deal {} hp!",usr.name,usr.damage);
                enemy.hp -= usr.damage as i8;

                if usr.hp <= 0{
                    println!("you lose!");
                    break;
                }
                if enemy.hp < 0 && usr.hp > 0 {
                    println!("you won!");
                    break;
                }
            }
            "2" => {break;}
            _ => {}
        };
    }
}