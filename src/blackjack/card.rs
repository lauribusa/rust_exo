#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    pub fn get_icon(&self) -> char{
        match &self {
            Suit::Spades => '♠',
            Suit::Hearts => '♡',
            Suit::Diamonds => '♢',
            Suit::Clubs => '♣',
        }
    }

    pub fn get_name(&self) -> String {
        match &self {
            Suit::Spades => String::from("Spades"),
            Suit::Hearts => String::from("Hearts"),
            Suit::Diamonds => String::from("Diamonds"),
            Suit::Clubs => String::from("Clubs"),
        }
    }
}

#[derive(PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8)
}

impl Rank {
    pub fn get_value(&self) -> u8{
        match &self {
            Rank::Ace => 1 as u8,
            Rank::King => 10 as u8,
            Rank::Queen => 10 as u8,
            Rank::Jack => 10 as u8,
            Rank::Number(v) => *v,
        }
    }

    pub fn get_name(&self) -> String {
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

pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn print_card(&self){
        let me = &self;
        println!(
            "[ {} of {} ] {} ({})",
            me.rank.get_name(), me.suit.get_name(), me.get_shorthand(), me.rank.get_value()
        );
    }

    pub fn get_shorthand(&self) -> String{
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

    pub fn new(suit: Suit, rank: Rank) -> Card {
        Card {
            rank,
            suit,
        }
    }
}