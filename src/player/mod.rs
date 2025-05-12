pub struct Character {
    pub hp: i8,
    pub max_hp: u8,
    pub name: String,
    pub damage: u8,
}

impl Character {
    pub fn new(hp: u8, name: String, damage: u8) -> Self {
        Self {
            hp: hp as i8,
            max_hp: hp,
            name,
            damage,
        }
    }

    pub fn stats(&self){
        println!();
        println!(" ==== {} ==== ",self.name);
        println!(" {} / {} HP ",self.hp,self.max_hp);
        println!(" {} DAMAGE",self.damage);
        let l = format!(" ==== {} ==== ",self.name).len();
        print!(" ");
        for _ in 1..l-1{
            print!("=")
        }
        print!(" ");
        println!();
    }
}
