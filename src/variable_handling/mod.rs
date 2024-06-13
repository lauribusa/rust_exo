const MATH_PI: f64 = 3.14159265359;

/*
var, fn: snake_case
const: PASCAL_CASE
everything else: CamelCase
*/

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
    // prints "true"
}

pub fn argument_func(number: &i32){
    println!("Here's a number: {}", number);
}

pub fn argument_func_return_value(score: &i32) -> &str{
    let over_threshold = "Over 160";
    let under_threshold = "Under 160";
    if score >= &160  {
        return over_threshold;
    }
    under_threshold
}

pub fn is_even(val: i32) -> bool{
    let is_pair = val % 2;
    is_pair == 0
}

pub fn is_odd(val: i32) -> bool{
    let is_pair = val % 2;
    is_pair != 0
}

pub fn even_or_odd(val: i32) -> String{
    let is_even= "Is Even".to_string();
    let is_odd = "Is odd".to_string();
    if val % 2 == 0{
        return is_even;
    }
    is_odd
}

pub fn add(operand1: i32, operand2: i32) -> i32{
    operand1 + operand2
}