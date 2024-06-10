fn main() {
    math_addition();
}

fn math_addition() {
    // cargo run X Y
    let num1 = std::env::args().nth(1).unwrap().parse::<i32>().unwrap();

    let num2 = std::env::args().nth(2).unwrap().parse::<i32>().unwrap();

    let added = num1 + num2;
    println!("printed with arguments in CLI: {added}");
}
