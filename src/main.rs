mod player;
mod game_manager;
use std::io::{self, Write,stdout};
use crossterm::{execute, terminal::{Clear, ClearType}};


fn game_start() -> player::Character {
    let mut name = String::new();
    println!("==== welcome ====");
    println!("what's your name?");
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut name)
        .expect("somethig went wrong");
    if name != "" {
        let p = player::Character::new(100, String::from(name.trim()), 12_u8);
        p
    } else {
        game_start()
    }
}

fn main() {
    let mut player = game_start();
    loop {
        let mut choice = String::new();
        println!("==== RustRpg ====");
        println!("#1. fight");
        println!("#2. stats");
        println!("#3. shop");
        println!("#4. inventory");
        println!("exit");
        while choice == "" {
            io::stdin().read_line(&mut choice).unwrap();
        }
        match choice.trim() {
            "1" => {
                 execute!(stdout(), Clear(ClearType::All)).unwrap();
                 print!("\x1B[H");
                game_manager::fight(&mut player);
            }
            "2" => {
                player.stats();
            }
            _=> {
                std::process::exit(0_i32)
            }
        };
    }
}
