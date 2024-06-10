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
    math_addition();
    argument_func(&88);
    let score = argument_func_return_value(&200);
    println!("Returned result is: {score}");
    let result = add(4, 6);
    println!("Returned argument function with sum value of: {result}");

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

fn math_addition() {
    // cargo run X Y
    let num1 = std::env::args().nth(1).unwrap().parse::<i32>().unwrap();

    let num2 = std::env::args().nth(2).unwrap().parse::<i32>().unwrap();

    let added = num1 + num2;
    println!("printed with arguments in CLI: {added}");
}

fn argument_func(number: &i32){
    println!("Here's a number: {}", number);
}

fn argument_func_return_value(score: &i32) -> &str{
    let over_threshold = "Profound retardation";
    let under_threshold = "Very autistic";
    if score > &160  {
        return over_threshold;
    }
    under_threshold
}

fn add(operand1: i32, operand2: i32) -> i32{
    let sum = operand1 + operand2;
    sum
}
