    
struct Rectangle {
    width: u8,
    height: u8
}

impl Rectangle {
    fn area(&self) -> u8 {
        self.width * self.height
    }

    // vérifie qu'un rectangle est plus gros qu'un autre
    fn is_bigger_than(&self, rec: &Rectangle) -> bool {
        self.area() > rec.area()
    }
}

pub fn struct_constructor(){
    let r = Rectangle {
        width: 10,
        height: 8
    };

    let r2 = Rectangle {
        width: 5,
        height: 8
    };
    let aire_2 = r2.area();

    println!("Aire 2 : {aire_2}");

    println!("Qui est le plus grand? r || r2");

    if r.is_bigger_than(&r2) {
        println!("C'est R");
    } else {
        println!("C'est R2");
    }
}

struct User {
    name: String,
    age: u8
}

pub fn user_builder() {
    let user = User {
        name: String::from("Marine"),
        age: 18
    };

    let mut user_2 = User {
        name: String::from("Jean Marie"),
        age: 95
    };

    println!("{}", user.name);
    println!("{}", user.age);

    user_2.age = 18;

    println!("{}", user_2.name);
    println!("{}", user_2.age);
}

struct Character {
    name: String,
    atk: i32,
    hp: i32,
}

impl Character {
    fn new(name: String, atk: i32, hp: i32) -> Character{
        Character {
            name,
            atk,
            hp
        }
    }

    fn take_damage(&mut self, damage: i32) {
        let mut life = self.hp;
        life -= &damage;
        if life.is_negative() {
            life = 0;
        }
        self.hp = life;
    }
}

pub fn battle_arena(){
    let mut red_corner = Character::new("Savage Ork".to_string(), 10, 40);
    let mut blue_corner = Character::new("Baratus".to_string(), 20, 30);

    loop {
        if red_corner.hp == 0 {
            println!("{} lost", red_corner.name);
            break;
        }

        if blue_corner.hp == 0 {
            println!("{} lost", blue_corner.name);
            break;
        }
        
        red_corner.take_damage(blue_corner.atk);
        print_battle(&red_corner, &blue_corner);
        blue_corner.take_damage(red_corner.atk);
        print_battle(&blue_corner, &red_corner);
    }
}

fn print_battle(red_corner: &Character, blue_corner: &Character){
    println!("{} took {} damage. {}",red_corner.name , blue_corner.atk, red_corner.hp);
}