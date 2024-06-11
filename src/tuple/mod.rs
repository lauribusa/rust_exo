use crate::random_number;

pub fn create_tuple(){
    // tuple cannot be iterated
    let infos: (&str, i32) = ("Jeanne", 18);

    println!("{}", infos.0);
    println!("{}", infos.1);

    let (nom, age) = ("Jean-Marie", 90);

    println!("{nom} {age}");
}

fn number_pairs() -> (u8, u8){
    let r1 = random_number::rand() as u8;
    let r2 = random_number::rand() as u8;
    (r1, r2)
}

pub fn calculate_pairs(){
    let result = number_pairs();
    let sum = result.0 + result.1;
    if sum == 7 || sum == 11 {
        println!("Result is 7 or 11: ({} , {}) ({sum})", result.0, result.1);
        return;
    }
    println!("Result is not 7 or 11: ({} , {}) ({sum})", result.0, result.1);
}