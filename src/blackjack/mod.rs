
mod card;
mod print;
mod game;

use card::*;
use print::*;
use game::*;

pub fn start() {
    let deck = setup_deck();
    print_title();
    print_options();
    loop {
        let input = read_input();
        if input == "1" {
            start_round(&deck);
        }

        if input == "2" {
            println!("Thanks for playing.");
            break;
        }
    }
}
