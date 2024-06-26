use crate::random_number::rand_max;

#[derive(PartialEq)]
struct Card {
    info: CardInfo,
    suit: Suit,
}

#[derive(PartialEq)]
struct CardInfo {
    name: String,
    value: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

fn build_deck() -> Vec<Card> {
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

fn create_suit_deck(suit: Suit) -> Vec<Card> {
    let mut cards: Vec<Card> = vec![];
    let card_values: [(&str, u8); 13] = [
        ("Ace", 1),
        ("Two", 2),
        ("Three", 3),
        ("Four", 4),
        ("Five", 5),
        ("Six", 6),
        ("Seven", 7),
        ("Eight", 8),
        ("Nine", 9),
        ("Ten", 10),
        ("Jack", 10),
        ("Queen", 10),
        ("King", 10),
    ];
    for card_value in card_values {
        let card_info = create_card_info(card_value.0, card_value.1);
        cards.push(create_card(card_info, suit));
    }
    cards
}

fn create_card_info(name: &str, value: u8) -> CardInfo {
    CardInfo {
        name: name.to_string(),
        value,
    }
}

fn create_card(card_info: CardInfo, suit: Suit) -> Card {
    Card {
        info: card_info,
        suit,
    }
}

fn display_card(card: &Card) -> String {
    let card_name = card.info.name.as_str();

    let suit = match card.suit {
        Suit::Spades => '♠',
        Suit::Hearts => '♡',
        Suit::Diamonds => '♢',
        Suit::Clubs => '♣',
    };

    let value = match card_name {
        "Ace" => "A",
        "Two" => "2",
        "Three" => "3",
        "Four" => "4",
        "Five" => "5",
        "Six" => "6",
        "Seven" => "7",
        "Eight" => "8",
        "Nine" => "9",
        "Ten" => "10",
        "Jack" => "J",
        "Queen" => "Q",
        "King" => "K",
        _ => "X"
    };

    
    let mut card_icon = "".to_string();
    card_icon.push_str(value);
    card_icon.push(suit);
    card_icon
}

pub fn run_game() {
    let mut deck = build_deck();
    print_title();
    print_options();
    loop {
        let input = read_input();
        if input == "1" {
            start_round(&mut deck);
        }

        if input == "2" {
            println!("Thanks for playing.");
            break;
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read");
    input.trim().to_string()
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

fn start_round(deck: &Vec<Card>) {
    println!("____ ROUND START ____");
    let mut cards_in_hand: u8 = 2;
    //print_deck(&deck);

    let mut hand: Vec<&Card> = vec![];
    let mut hand_value: u8 = 0;

    loop {
        let card = get_card(&deck);
        /*if hand.contains(&card) {
            println!("Card [ {} of {:?} ] ({}) already in hand, retrying...", card.info.name, card.suit, card.info.value);
            continue;
        }*/
        hand.push(card);
        cards_in_hand -= 1;
        if cards_in_hand <= 0 {
            hand_value = get_hand_score(&mut hand);
            break;
        }
    }

    print_hand_value(&hand, hand_value);
    

    loop {
        show_commands();
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
                println!("Player bust with {}. DEALER WINS.", hand_value);
                print_options();
                break;
            }
            print_hand_value(&hand, hand_value);
        }

        if input == "2" {
            println!("Stay...");
            let mut dealer_hand: Vec<&Card> = vec![];
            let mut dealer_score: u8 = 0;

            for i in 0..2 {
                let card = get_card(&deck);
                dealer_hand.push(card);
            }

            dealer_score = get_hand_score(&mut dealer_hand);
            if dealer_score > 21 {
                print_game_hands(&hand, hand_value, &dealer_hand, dealer_score);
                println!("Dealer bust with {}. PLAYER WINS.", dealer_score);
                print_options();
                break;
            }

            loop {
                if dealer_score > 21 {
                    print_game_hands(&hand, hand_value, &dealer_hand, dealer_score);
                    println!("Dealer bust with {}. PLAYER WINS.", dealer_score);
                    break;
                } else
                if dealer_score < 18 {
                    let card = get_card(&deck);
                    dealer_hand.push(card);
                    dealer_score = get_hand_score(&mut dealer_hand);
                    continue;
                } else {
                    if dealer_score > hand_value {
                        print_game_hands(&hand, hand_value, &dealer_hand, dealer_score);
                        println!("DEALER WINS.");
                        break;
                    }

                    if dealer_score < hand_value {
                        print_game_hands(&hand, hand_value, &dealer_hand, dealer_score);
                        println!("PLAYER WINS.");
                        break;
                    }

                    if dealer_score == hand_value {
                        print_game_hands(&hand, hand_value, &dealer_hand, dealer_score);
                        println!("DRAW.");
                        break;
                    }
                }
            }
            print_options();
            break;
        }
    }
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

fn print_hand(hand: &Vec<&Card>) {
    
    for card in hand {
        let card_icon = display_card(card);
        println!(
            "[ {} of {:?} ] {} ({})",
            card.info.name, card.suit,card_icon, card.info.value
        );
    }
}

fn print_title(){
    println!(r"__________.____       _____  _________  ____  __.    ____.  _____  _________  ____  __.");
    println!(r"\______   \    |     /  _  \ \_   ___ \|    |/ _|   |    | /  _  \ \_   ___ \|    |/ _|");
    println!(r" |    |  _/    |    /  /_\  \/    \  \/|      <     |    |/  /_\  \/    \  \/|      <  ");
    println!(r" |    |   \    |___/    |    \     \___|    |  \/\__|    /    |    \     \___|    |  \ ");
    println!(r" |______  /_______ \____|__  /\______  /____|__ \________\____|__  /\______  /____|__ \");
    println!(r"        \/        \/       \/        \/        \/                \/        \/        \/");
}

fn get_hand_score(hand: &mut Vec<&Card>) -> u8 {
    let mut score: u8 = 0;
    let mut owned_aces: u8 = 0;
    for card in hand {
        let card_value = card.info.value;
        if card_value == 1 {
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

fn hit<'a>(deck: &'a Vec<Card>, hand: &mut Vec<&'a Card>) {
    loop {
        let card = get_card(&deck);

        hand.push(card);
        break;
    }
}

fn deal_hand(deck: &mut Vec<Card>) {
    let mut cards_in_hand: u8 = 5;
    //print_deck(&deck);

    let mut hand: Vec<&Card> = vec![];

    loop {
        let card = get_card(&deck);

        if hand.contains(&card) {
            println!(
                "Card [ {} of {:?} ] ({}) already in hand, retrying...",
                card.info.name, card.suit, card.info.value
            );
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
        println!(
            "[ {} of {:?} ] ({})",
            card.info.name, card.suit, card.info.value
        );
    }
}

fn print_deck(deck: &Vec<Card>) {
    for card in deck {
        println!(
            "{} of {:?} ({})",
            card.info.name, card.suit, card.info.value
        );
    }
}

fn get_card(deck: &Vec<Card>) -> &Card {
    let random_card_index = rand_max(deck.len() as u128);
    let card = &deck[random_card_index as usize];
    card
}

//remove_card(&mut deck, &card);

fn remove_card(deck: &mut Vec<Card>, card: &Card) {
    // Panic if no such element is found
    deck.remove(
        deck.iter()
            .position(|x| *x == *card)
            .expect("Card not found"),
    );

    // Ignore if no such element is found
    /*if let Some(pos) = deck.iter().position(|x| *x == *card) {
        deck.remove(pos);
    }*/
}
