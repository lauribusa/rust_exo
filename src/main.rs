const MATH_PI: f64 = 3.14159265359;
/*
var, fn: snake_case
const: PASCAL_CASE
everything else: CamelCase
*/

fn main() {
    let number = 1;
    println!("Printing implicit variable in main: {number}");
    println!("PI equals to {MATH_PI}");
    print_my_stuff();
    mutable_variable();
    shadowing_variable();
    argument_func(&88);
    let score = argument_func_return_value(&200);
    println!("Returned result is: {score}");
    let result = add(4, 6);
    println!("Returned argument function with sum value of: {result}");
    let mut pwr = is_even(40);
    println!("is 40 even? {pwr}");
    pwr = is_even(3);
    println!("is 3 even? {pwr}");
    pwr = is_odd(55);
    println!("is 55 odd? {pwr}");
    let txt = even_or_odd(100);
    println!("{txt}");

}

fn print_my_stuff() {
    let a_number: i64 = 32;
    println!("Printing a i64 number: {a_number}");
}

fn mutable_variable() {
    let mut mutable = 20;
    mutable = 10;
    println!("Variable used to be 20, now: {mutable}");
}

fn shadowing_variable() {
    let mut mutable: i32 = 20;
    mutable = 10;

    let mutable: bool = true;
    println!("Variable used to be i32 with value of 20, now: {mutable}");
    // prints "true"
}

fn argument_func(number: &i32){
    println!("Here's a number: {}", number);
}

fn argument_func_return_value(score: &i32) -> &str{
    let over_threshold = "Over 160";
    let under_threshold = "Under 160";
    if score >= &160  {
        return over_threshold;
    }
    under_threshold
}

fn is_even(val: i32) -> bool{
    let is_pair = val % 2;
    is_pair == 0
}

fn is_odd(val: i32) -> bool{
    let is_pair = val % 2;
    is_pair != 0
}

fn even_or_odd(val: i32) -> String{
    let is_even= "Is Even".to_string();
    let is_odd = "Is odd".to_string();
    if val % 2 == 0{
        return is_even;
    }
    is_odd
}

fn add(operand1: i32, operand2: i32) -> i32{
    operand1 + operand2
}

fn switch_de_nintendo(){

}
