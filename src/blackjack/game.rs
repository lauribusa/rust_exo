use std::time::{SystemTime, UNIX_EPOCH};

use crate::blackjack::*;

use super::{Card, Rank, Suit};

fn rand_max(max: u128) -> u32 {
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let nanos = since_epoch.as_nanos();

    (nanos % max) as u32
}

pub fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Couldn't read");
    input.trim().to_string()
}

pub fn setup_deck() -> Vec<Card> {
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

pub fn start_round(deck: &[Card]) {
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
