use std::env::args;

fn main() {
    let number = 1;
    println!("Printing implicit variable in main: {number}");
    print_my_stuff();
    mutable_variable();
    shadowing_variable();
    math_addition();
}

pub fn print_my_stuff() {
    let a_number: i64 = 32;
    println!("Printing a i64 number: {a_number}");
}

pub fn mutable_variable() {
    let mut mutable = 20;
    mutable = 10;
    println!("Variable used to be 20, now: {mutable}");
}

pub fn shadowing_variable() {
    let mut mutable: i32 = 20;
    mutable = 10;

    let mutable: bool = true;
    println!("Variable used to be i32 with value of 20, now: {mutable}");
}

pub fn math_addition() {
    // cargo run X Y
    let num1 = std::env::args().nth(1).unwrap().parse::<i32>().unwrap();

    let num2 = std::env::args().nth(2).unwrap().parse::<i32>().unwrap();

    let added = num1 + num2;
    println!("printed with arguments in CLI: {added}");
}
