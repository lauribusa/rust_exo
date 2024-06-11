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

