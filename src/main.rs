// mod r#array;
// mod random_number;
// mod r#loop;
// mod read_terminal;
// mod tuple;
// mod memory;
// mod strings;
// mod r#struct;

// mod blackjack;
// use blackjack::start;

// mod tuple_struct;
// mod r#enum;
// mod options;

// mod mini_rpg;
//use crate::mini_rpg::player::*;

use bank_system::bank;

mod error_handling;
mod bank_system;

fn main() {
    bank();
    // card_game::run_game();
    // memory::todo_list_creation();
    // strings::print_string();
    // strings::exo_string_2("rougegarde", "Race de Skyrim");
    // start();
    // r#struct::battle_arena();
    // tuple_struct::tuple_struct_constructor();
    // tuple_struct::create_password_and_compare();
    // r#enum::create_orc_faction();
    // options::create_user_table();
    // options::create_options_table();
    // imported from mini_rpg::player
    // let mm = PlayerCharacter::new("Baratus");
    error_handling::banking_system();
}
