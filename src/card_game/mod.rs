use crate::random_number::rand_max;

#[derive(PartialEq)]
struct Card {
    info: CardInfo,
    suit: Suit,
}

#[derive(PartialEq)]
struct CardInfo{
    name: String,
    value: u8
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs
}

fn build_deck() -> Vec<Card>{
    let mut new_deck = vec![];
    let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
    for suit in suits.iter() {
        let cards = create_suit_deck(suit.to_owned());
        for card in cards {
            new_deck.push(card);
        }
    }
    new_deck
}

fn create_suit_deck(suit: Suit) -> Vec<Card>{
    let mut cards: Vec<Card> = vec![];
    let card_values: [(&str, u8); 13] = [("Ace", 1), ("Two", 2), ("Three", 3), ("Four", 4), ("Five", 5), ("Six", 6), ("Seven", 7), ("Eight", 8), ("Nine", 9), ("Ten", 10), ("Jack", 10), ("Queen", 10), ("King", 10)];
    for card_value in card_values {
        let card_info = create_card_info(card_value.0, card_value.1);
        cards.push(create_card(card_info, suit));
    }
    cards
}

fn create_card_info(name: &str, value: u8) -> CardInfo{
    CardInfo {
        name: name.to_string(),
        value
    }
}

fn create_card(card_info: CardInfo, suit: Suit) -> Card{
    Card { info: card_info, suit }
}

pub fn run_game(){
    let deck = build_deck();
    show_commands();
    loop {
        let input = read_input();
        if input == "1" {
            deal_hand(&deck);
        }

        if input == "2" {
            println!("Thanks for playing.");
            break;
        }

        if input == "3" {
            show_commands();
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Couldn't read");
    input.trim().to_string()
}

fn show_commands(){
    println!("___________");
    println!("List of commands:");
    println!("1. Deal hand");
    println!("2. Quit");
    println!("3. Print these commands");
    println!("___________");
}

fn print_options(){
    println!("___________");
    println!("List of commands:");
    println!("1. Begin round");
    println!("2. Print these commands");
    println!("3. Quit");
    println!(" ----- ");
    println!(" -- In round --");
    println!("1. Hit");
    println!("2. Stay");
    println!("___________");
}

fn start_round(deck: &Vec<Card>){
    let mut cards_in_hand: u8 = 2;
    //print_deck(&deck);

    let mut hand : Vec<&Card> = vec![];

    loop {
        let card = get_card(&deck);
        if hand.contains(&card) {
            println!("Card [ {} of {:?} ] ({}) already in hand, retrying...", card.info.name, card.suit, card.info.value);
            continue;
        }
        hand.push(card);
        cards_in_hand -= 1;
        if cards_in_hand <= 0 {
            break;
        }
    }

    println!("Hand contains:");
    for card in &hand {
        println!("[ {} of {:?} ] ({})", card.info.name, card.suit, card.info.value);
    }

    loop {
        let input = read_input();
        let mut hand_value: u8 = 0;
        let mut owned_aces: i32 = 0;
        for card in hand.iter() {
            if card.info.value == 1 {
                owned_aces += 1;
                continue;
            }
            hand_value += card.info.value;
        }
        if input == "1" {
            println!("Hit...");
            hit(deck, &mut hand);

        }

        if input == "2" {
            println!("Stay...");
            break;
        }
    }
}

fn check_hand_value() {}

fn hit<'a>(deck: &'a Vec<Card>, hand: &mut Vec<&'a Card>) {
    loop {
        let card = get_card(&deck);
        if hand.contains(&card) {
            println!("Card [ {} of {:?} ] ({}) already in hand, retrying...", card.info.name, card.suit, card.info.value);
            continue;
        }
        hand.push(card);
        break;
    }
}

fn round_hit(){

}

fn round_stay(){

}

fn deal_hand(deck: &mut Vec<Card>){
    let mut cards_in_hand: u8 = 5;
    //print_deck(&deck);

    let mut hand : Vec<&Card> = vec![];

    loop {
        let card = get_card(&deck);
        
        if hand.contains(&card) {
            println!("Card [ {} of {:?} ] ({}) already in hand, retrying...", card.info.name, card.suit, card.info.value);
            continue;
        }
        hand.push(card);
        cards_in_hand -= 1;
        if cards_in_hand <= 0 {
            break;
        }
    }
    println!("Hand contains:");
    for card in hand {
        println!("[ {} of {:?} ] ({})", card.info.name, card.suit, card.info.value);
    }
}

fn print_deck(deck: &Vec<Card>) {
    for card in deck {
        println!("{} of {:?} ({})", card.info.name, card.suit, card.info.value);
    }
}

fn get_card(deck: &Vec<Card>) -> &Card {
    let random_card_index = rand_max(deck.len() as u128);
    let card = &deck[random_card_index as usize];
    card
}

//remove_card(&mut deck, &card);

fn remove_card(deck: &mut Vec<Card>, card: &Card){
    // Panic if no such element is found
    deck.remove(deck.iter().position(|x| *x == *card).expect("Card not found"));

// Ignore if no such element is found
   /*if let Some(pos) = deck.iter().position(|x| *x == *card) {
        deck.remove(pos);
    }*/
}

