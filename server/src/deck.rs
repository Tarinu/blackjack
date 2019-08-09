use std::fmt;
use rand::{
    thread_rng,
    seq::SliceRandom
};

pub enum CardSuit {
    Heart,
    Spade,
    Club,
    Diamond
}

impl fmt::Display for CardSuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match *self {
            CardSuit::Heart => "hearts",
            CardSuit::Club => "clubs",
            CardSuit::Spade => "spades",
            CardSuit::Diamond => "diamonds"
        };

        write!(f, "{}", text)
    }
}

enum CardValue {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

impl fmt::Display for CardValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match *self {
            CardValue::Ace => "ace",
            CardValue::Two => "two",
            CardValue::Three => "three",
            CardValue::Four => "four",
            CardValue::Five => "five",
            CardValue::Six => "six",
            CardValue::Seven => "seven",
            CardValue::Eight => "eight",
            CardValue::Nine => "nine",
            CardValue::Ten => "ten",
            CardValue::Jack => "jack",
            CardValue::Queen => "queen",
            CardValue::King => "king"
        };

        write!(f, "{}", text)
    }
}

pub struct Card<'a> {
    suit: &'a CardSuit,
    value: &'a CardValue
}

impl<'a> fmt::Display for Card<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }
}

pub struct Deck<'a> {
    cards: Vec<Card<'a>>
}

impl<'a> Deck<'a> {
    pub fn new() -> Deck<'a> {
        let mut cards = Vec::new();
        static CARDSUITS: [CardSuit; 4] = [CardSuit::Heart, CardSuit::Spade, CardSuit::Club, CardSuit::Diamond];
        static CARDVALUES: [CardValue; 13] = [
            CardValue::Ace,
            CardValue::Two,
            CardValue::Three,
            CardValue::Four,
            CardValue::Five,
            CardValue::Six,
            CardValue::Seven,
            CardValue::Eight,
            CardValue::Nine,
            CardValue::Ten,
            CardValue::Jack,
            CardValue::Queen,
            CardValue::King
        ];

        for suit in CARDSUITS.into_iter() {
            for value in CARDVALUES.into_iter() {
                cards.push(Card {
                    suit,
                    value
                });
            }
        }


        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        Deck {
            cards
        }
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Returns the number of cards still left in the deck
    pub fn len(&self) -> usize {
        self.cards.len()
    }
}
