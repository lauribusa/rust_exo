pub fn print_win(){
    println!("__________________");
    println!("### PLAYER WIN ###");
    println!("__________________");
}

pub fn print_lose(){
    println!("___________________");
    println!("### PLAYER LOSE ###");
    println!("___________________");
}

pub fn print_draw(){
    println!("_____________");
    println!("### DRAW. ###");
    println!("_____________");
}

pub fn print_title() {
    println!(r"                                                                                            ");
    println!(r"                                                                                            ");
    println!(r"   /$$$$$$$  /$$        /$$$$$$   /$$$$$$  /$$   /$$    /$$$$$  /$$$$$$   /$$$$$$  /$$   /$$");
    println!(r"  | $$__  $$| $$       /$$__  $$ /$$__  $$| $$  /$$/   |__  $$ /$$__  $$ /$$__  $$| $$  /$$/");
    println!(r"  | $$  \ $$| $$      | $$  \ $$| $$  \__/| $$ /$$/       | $$| $$  \ $$| $$  \__/| $$ /$$/ ");
    println!(r"  | $$$$$$$ | $$      | $$$$$$$$| $$      | $$$$$/        | $$| $$$$$$$$| $$      | $$$$$/  ");
    println!(r"  | $$__  $$| $$      | $$__  $$| $$      | $$  $$   /$$  | $$| $$__  $$| $$      | $$  $$  ");
    println!(r"  | $$  \ $$| $$      | $$  | $$| $$    $$| $$\  $$ | $$  | $$| $$  | $$| $$    $$| $$\  $$ ");
    println!(r"  | $$$$$$$/| $$$$$$$$| $$  | $$|  $$$$$$/| $$ \  $$|  $$$$$$/| $$  | $$|  $$$$$$/| $$ \  $$");
    println!(r"  |_______/ |________/|__/  |__/ \______/ |__/  \__/ \______/ |__/  |__/ \______/ |__/  \__/");
    println!(r"                                                                                            ");
    println!(r"                                                                                            ");
}


pub fn show_commands() {
    println!("___________");
    println!("List of commands:");
    println!("1. Hit");
    println!("2. Stay");
    println!("___________");
}

pub fn print_options() {
    println!("___________");
    println!("List of commands:");
    println!("1. Begin round");
    println!("2. Quit game");
    println!("___________");
}

pub fn print_hand(hand: &Vec<&super::Card>) {
    for card in hand {
        card.print_card();
    }
}

pub fn print_hand_value(hand: &Vec<&super::Card>, hand_value: u8) {
    print_hand(hand);
    println!("Total: {}", hand_value);
}

pub fn print_game_hands(hand: &Vec<&super::Card>, hand_value: u8, dealer_hand: &Vec<&super::Card>, dealer_score: u8) {
    println!("Player hand:");
    print_hand_value(hand, hand_value);
    println!("Dealer hand:");
    print_hand_value(dealer_hand, dealer_score);
}