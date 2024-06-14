use std::fmt::Display;
use std::fs;
use std::io;
use std::num;

struct Vector2{
    x: i32,
    y: i32
}

// basically like an interface in another language
trait Animate {
    fn r#move(&mut self, position: Vector2);
}

struct Creature {
    name: String,
    position: Vector2
}

impl Animate for Creature {
    fn r#move(&mut self, position: Vector2) {
        let mut new_pos = &mut self.position;
        self.position.x += position.x;
        self.position.y += position.y;
    }
}

fn create_creature(){
    make_creature_move(Creature{name:String::from("Pet"), position: Vector2 {x: 0, y:0}});
}

// same as fn function<T: is Animate>(entity: T)
fn make_creature_move<T: Animate>(entity: T){

}

trait Alive {
    fn is_alive(&self) -> bool;
}

trait Animal: Alive {
    fn get_age(&self) -> i32;

    fn speak(&self) -> Option<i32> {
        if self.is_alive() {
            return Some(self.get_age());
        }else {
            return None;
        }
    }
}

struct Password(String);

impl From<i32> for Password {
    fn from(value: i32) -> Self {
        Password(String::new())
    }
}

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(error: num::ParseIntError) -> Self {
        CliError::ParseError(error)
    }
}

struct EUR(i32);

struct USD(i32);

impl From<EUR> for USD {
    fn from(value: EUR) -> Self {
        USD(value.0 / 2)
    }
}

impl From<USD> for EUR {
    fn from(value: USD) -> Self {
        EUR(value.0 * 2)
    }
}

impl Display for EUR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(println!("{}€", self.0))
    }
}

impl Display for USD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(println!("${}", self.0))
    }
}


pub fn convert_currency(){
    let usd = USD(100);
    let eur = EUR(55);

    let c_usd: USD = eur.into();
    let c_eur: EUR = usd.into();

    println!("{c_usd}");
    println!("{c_eur}");
}

    
trait Compare<T: PartialEq> {
    fn is_equal(&self, other: &T) -> bool;
}

    
struct Circle {
    radius: f64,
}


impl Compare<f64> for Circle {
    fn is_equal(&self, other: &f64) -> bool {
        return *other == self.radius;
    }
}

fn implement_compare() {
    let circle = Circle {
        radius: 15_f64
    };

    let is_equal = circle.is_equal(&15_f64);
    
    println!("{}", is_equal);
}
