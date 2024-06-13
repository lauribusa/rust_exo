use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    fn get_icon(&self) -> char{
        match &self {
            Suit::Spades => '♠',
            Suit::Hearts => '♡',
            Suit::Diamonds => '♢',
            Suit::Clubs => '♣',
        }
    }

    fn get_name(&self) -> String {
        match &self {
            Suit::Spades => String::from("Spades"),
            Suit::Hearts => String::from("Hearts"),
            Suit::Diamonds => String::from("Diamonds"),
            Suit::Clubs => String::from("Clubs"),
        }
    }
}

#[derive(PartialEq)]
enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8)
}

impl Rank {
    fn get_value(&self) -> u8{
        match &self {
            Rank::Ace => 1 as u8,
            Rank::King => 10 as u8,
            Rank::Queen => 10 as u8,
            Rank::Jack => 10 as u8,
            Rank::Number(v) => *v,
        }
    }

    fn get_name(&self) -> String {
        match &self {
            Rank::Ace => "Ace".to_string(),
            Rank::King => "King".to_string(),
            Rank::Queen => "Queen".to_string(),
            Rank::Jack => "Jack".to_string(),
            Rank::Number(num) => num.to_string(),
        }
    }

    fn get_ace_value(&self, score: u8) -> u8 {
        match &self {
            Rank::Ace => {
                if &score + 11 > 21 {
                    1
                } else {
                    11
                }
            }
            _ => self.get_value()
        }
    }
}

struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn print_card(&self){
        let me = &self;
        println!(
            "[ {} of {} ] {} ({})",
            me.rank.get_name(), me.suit.get_name(), me.get_shorthand(), me.rank.get_value()
        );
    }

    fn get_shorthand(&self) -> String{
        let me = &self;
        let suit = me.suit.get_icon();
        let rank = &me.rank;
        let mut shorthand = match rank {
            Rank::Ace => String::from("A"),
            Rank::King => String::from("K"),
            Rank::Queen => String::from("Q"),
            Rank::Jack => String::from("J"),
            Rank::Number(num) => num.to_string(),
        };
        shorthand.push(suit);
        shorthand
    }

    fn new(suit: Suit, rank: Rank) -> Card {
        Card {
            rank,
            suit,
        }
    }
}

fn print_win(){
    println!("__________________");
    println!("### PLAYER WIN ###");
    println!("__________________");
}

fn print_lose(){
    println!("___________________");
    println!("### PLAYER LOSE ###");
    println!("___________________");
}

fn print_draw(){
    println!("_____________");
    println!("### DRAW. ###");
    println!("_____________");
}

fn print_title() {
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

fn print_hand(hand: &Vec<&Card>) {
    for card in hand {
        card.print_card();
    }
}

fn print_card(card: &Card) -> String {
    let card_name = card.get_shorthand();
    card_name
}

fn print_hand_value(hand: &Vec<&Card>, hand_value: u8) {
    print_hand(hand);
    println!("Total: {}", hand_value);
}

fn print_game_hands(hand: &Vec<&Card>, hand_value: u8, dealer_hand: &Vec<&Card>, dealer_score: u8) {
    println!("Player hand:");
    print_hand_value(hand, hand_value);
    println!("Dealer hand:");
    print_hand_value(dealer_hand, dealer_score);
}

fn show_commands() {
    println!("___________");
    println!("List of commands:");
    println!("1. Hit");
    println!("2. Stay");
    println!("___________");
}

fn print_options() {
    println!("___________");
    println!("List of commands:");
    println!("1. Begin round");
    println!("2. Quit game");
    println!("___________");
}

fn rand_max(max: u128) -> u32 {
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let nanos = since_epoch.as_nanos();

    (nanos % max) as u32
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read");
    input.trim().to_string()
}

fn setup_deck() -> Vec<Card> {
    let mut new_deck = vec![];
    let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
    for suit in suits.iter() {
        let cards = create_deck(suit.to_owned());
        for card in cards {
            new_deck.push(card);
        }
    }
    new_deck
}

fn create_deck(suit: Suit) -> Vec<Card> {
    let mut cards: Vec<Card> = vec![];
    cards.push(Card::new(suit, Rank::King));
    cards.push(Card::new(suit, Rank::Queen));
    cards.push(Card::new(suit, Rank::Jack));
    for i in 0..9 {
        let val = 10 - i;
        cards.push(Card::new(suit, Rank::Number(val)));
    }
    cards.push(Card::new(suit, Rank::Ace));
    cards
}

pub fn run_game() {
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

fn hit<'a>(deck: &'a [Card], hand: &mut Vec<&'a Card>) {
    let card = get_card(deck);
    hand.push(card);
}

fn get_hand_score(hand: &mut Vec<&Card>) -> u8 {
    let mut score: u8 = 0;
    let mut owned_aces: u8 = 0;
    for card in hand {
        let card_value = card.rank.get_value();
        if card.rank == Rank::Ace {
            owned_aces += 1;
            continue;
        }
        score += card_value;
    }
    for _ace in 0..owned_aces {
        if score + 11 <= 21 {
            score += 11;
        } else {
            score += 1;
        }
    }
    score
}

fn get_card(deck: &[Card]) -> &Card {
    let random_card_index = rand_max(deck.len() as u128);
    &deck[random_card_index as usize]
}

fn start_round(deck: &[Card]) {
    println!("____ ROUND START ____");
    let mut cards_in_hand: u8 = 2;

    let mut hand: Vec<&Card> = vec![];
    let mut hand_value: u8 = 0;

    loop {
        let card = get_card(deck);
        hand.push(card);
        cards_in_hand -= 1;
        if cards_in_hand == 0 {
            hand_value = get_hand_score(&mut hand);
            break;
        }
    }

    print_hand_value(&hand, hand_value);

    loop {
        if hand_value == 21 {
            println!("Maximum score (21) reached. Press any key to continue...");
        } else {
            show_commands();
        }

        let mut input = read_input();

        if hand_value == 21 {
            input = "2".to_string();
        }

        if input == "1" {
            println!("Hit...");
            hit(deck, &mut hand);
            hand_value = get_hand_score(&mut hand);
            if hand_value > 21 {
                print_hand_value(&hand, hand_value);
                println!("Player bust with {}.", hand_value);
                print_lose();
                print_options();
                break;
            }
            print_hand_value(&hand, hand_value);
        }

        if input == "2" {
            println!("Stay...");
            let mut dealer_hand: Vec<&Card> = vec![];
            let mut dealer_score: u8 = 0;

            for _ in 0..2 {
                let card = get_card(deck);
                dealer_hand.push(card);
            }

            dealer_score = get_hand_score(&mut dealer_hand);
            if dealer_score > 21 {
                print_game_hands(&hand, hand_value, &dealer_hand, dealer_score);
                println!("Dealer bust with {}.", dealer_score);
                print_win();
                print_options();
                break;
            }

            loop {
                if dealer_score > 21 {
                    print_game_hands(&hand, hand_value, &dealer_hand, dealer_score);
                    println!("Dealer bust with {}.", dealer_score);
                    print_win();
                    break;
                } else if dealer_score < 18 {
                    let card = get_card(deck);
                    dealer_hand.push(card);
                    dealer_score = get_hand_score(&mut dealer_hand);
                    continue;
                } else {
                    if dealer_score > hand_value {
                        print_game_hands(&hand, hand_value, &dealer_hand, dealer_score);
                        print_lose();
                        break;
                    }

                    if dealer_score < hand_value {
                        print_game_hands(&hand, hand_value, &dealer_hand, dealer_score);
                        print_win();
                        break;
                    }

                    if dealer_score == hand_value {
                        print_game_hands(&hand, hand_value, &dealer_hand, dealer_score);
                        print_draw();
                        break;
                    }
                }
            }
            print_options();
            break;
        }
    }
}
