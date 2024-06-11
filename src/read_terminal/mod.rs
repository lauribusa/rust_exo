use crate::random_number;
use std::io;

fn read_terminal() -> u32{
    let mut input = String::new();
 
    io::stdin().read_line(&mut input)
        .expect("Fail");

   input.trim().parse::<u32>().unwrap()
}

pub fn read_random(){
    let result = random_number::rand();

    loop {
        let user_input = read_terminal();

        if result == user_input {
            println!("Number found");
            break;
        }

        if user_input < result {
            println!("It's more");
        }else{
            println!("It's less");
        }
    }
}

pub fn read_number_random(){
    let result = random_number::rand();
    let mut user_inputs: Vec<u32> = vec![];
    let mut loop_count = 10;

    loop {
        let user_input = read_terminal();
        user_inputs.push(user_input);

        if result == user_input {
            println!("Number found");
            break;
        }

        if user_input < result {
            println!("It's more");
        }else{
            println!("It's less");
        }
        loop_count -= 1;
        if loop_count <= 0 {
            println!("Out of tries.");
            break;
        }
        println!("Number of tries left: {loop_count}");
    }

    let mut result_string: String = "".to_string();
    for input in user_inputs.iter() {
        let w = "[".to_string() + &input.to_string() + &"] ".to_string();
        result_string += &w;
    }
    println!("User numbers: {result_string}");
    
}

