use std::fmt;
use rand::{
    thread_rng,
    seq::SliceRandom
};

#[derive(PartialEq)]
enum CardSuit {
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

#[derive(PartialEq)]
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

#[derive(PartialEq)]
pub struct Card {
    suit: CardSuit,
    value: CardValue
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }
}

pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = vec![
            Card { suit: CardSuit::Heart, value: CardValue::Ace },
            Card { suit: CardSuit::Heart, value: CardValue::Two },
            Card { suit: CardSuit::Heart, value: CardValue::Three },
            Card { suit: CardSuit::Heart, value: CardValue::Four },
            Card { suit: CardSuit::Heart, value: CardValue::Five },
            Card { suit: CardSuit::Heart, value: CardValue::Six },
            Card { suit: CardSuit::Heart, value: CardValue::Seven },
            Card { suit: CardSuit::Heart, value: CardValue::Eight },
            Card { suit: CardSuit::Heart, value: CardValue::Nine },
            Card { suit: CardSuit::Heart, value: CardValue::Ten },
            Card { suit: CardSuit::Heart, value: CardValue::Jack },
            Card { suit: CardSuit::Heart, value: CardValue::Queen },
            Card { suit: CardSuit::Heart, value: CardValue::King },
            Card { suit: CardSuit::Spade, value: CardValue::Ace },
            Card { suit: CardSuit::Spade, value: CardValue::Two },
            Card { suit: CardSuit::Spade, value: CardValue::Three },
            Card { suit: CardSuit::Spade, value: CardValue::Four },
            Card { suit: CardSuit::Spade, value: CardValue::Five },
            Card { suit: CardSuit::Spade, value: CardValue::Six },
            Card { suit: CardSuit::Spade, value: CardValue::Seven },
            Card { suit: CardSuit::Spade, value: CardValue::Eight },
            Card { suit: CardSuit::Spade, value: CardValue::Nine },
            Card { suit: CardSuit::Spade, value: CardValue::Ten },
            Card { suit: CardSuit::Spade, value: CardValue::Jack },
            Card { suit: CardSuit::Spade, value: CardValue::Queen },
            Card { suit: CardSuit::Spade, value: CardValue::King },
            Card { suit: CardSuit::Club, value: CardValue::Ace },
            Card { suit: CardSuit::Club, value: CardValue::Two },
            Card { suit: CardSuit::Club, value: CardValue::Three },
            Card { suit: CardSuit::Club, value: CardValue::Four },
            Card { suit: CardSuit::Club, value: CardValue::Five },
            Card { suit: CardSuit::Club, value: CardValue::Six },
            Card { suit: CardSuit::Club, value: CardValue::Seven },
            Card { suit: CardSuit::Club, value: CardValue::Eight },
            Card { suit: CardSuit::Club, value: CardValue::Nine },
            Card { suit: CardSuit::Club, value: CardValue::Ten },
            Card { suit: CardSuit::Club, value: CardValue::Jack },
            Card { suit: CardSuit::Club, value: CardValue::Queen },
            Card { suit: CardSuit::Club, value: CardValue::King },
            Card { suit: CardSuit::Diamond, value: CardValue::Ace },
            Card { suit: CardSuit::Diamond, value: CardValue::Two },
            Card { suit: CardSuit::Diamond, value: CardValue::Three },
            Card { suit: CardSuit::Diamond, value: CardValue::Four },
            Card { suit: CardSuit::Diamond, value: CardValue::Five },
            Card { suit: CardSuit::Diamond, value: CardValue::Six },
            Card { suit: CardSuit::Diamond, value: CardValue::Seven },
            Card { suit: CardSuit::Diamond, value: CardValue::Eight },
            Card { suit: CardSuit::Diamond, value: CardValue::Nine },
            Card { suit: CardSuit::Diamond, value: CardValue::Ten },
            Card { suit: CardSuit::Diamond, value: CardValue::Jack },
            Card { suit: CardSuit::Diamond, value: CardValue::Queen },
            Card { suit: CardSuit::Diamond, value: CardValue::King },
        ];

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

    #[allow(dead_code)]
    fn contains(&self, card: &Card) -> bool {
        self.cards.contains(card)
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn deck_has_correct_number_of_cards() {
        assert_eq!(52, Deck::new().len());
    }

    #[test]
    fn deck_pop_removes_card() {
        let mut deck = Deck::new();
        let len = deck.len();
        let card = deck.pop();
        assert_eq!(len - 1, deck.len());
        
        assert!(card.is_some());
        let card = card.unwrap();
        assert!(!deck.contains(&card));
    }

    #[test]
    fn deck_cards_are_unique() {
        let cards = [
            Card { suit: CardSuit::Heart, value: CardValue::Ace },
            Card { suit: CardSuit::Heart, value: CardValue::Two },
            Card { suit: CardSuit::Heart, value: CardValue::Three },
            Card { suit: CardSuit::Heart, value: CardValue::Four },
            Card { suit: CardSuit::Heart, value: CardValue::Five },
            Card { suit: CardSuit::Heart, value: CardValue::Six },
            Card { suit: CardSuit::Heart, value: CardValue::Seven },
            Card { suit: CardSuit::Heart, value: CardValue::Eight },
            Card { suit: CardSuit::Heart, value: CardValue::Nine },
            Card { suit: CardSuit::Heart, value: CardValue::Ten },
            Card { suit: CardSuit::Heart, value: CardValue::Jack },
            Card { suit: CardSuit::Heart, value: CardValue::Queen },
            Card { suit: CardSuit::Heart, value: CardValue::King },
            Card { suit: CardSuit::Spade, value: CardValue::Ace },
            Card { suit: CardSuit::Spade, value: CardValue::Two },
            Card { suit: CardSuit::Spade, value: CardValue::Three },
            Card { suit: CardSuit::Spade, value: CardValue::Four },
            Card { suit: CardSuit::Spade, value: CardValue::Five },
            Card { suit: CardSuit::Spade, value: CardValue::Six },
            Card { suit: CardSuit::Spade, value: CardValue::Seven },
            Card { suit: CardSuit::Spade, value: CardValue::Eight },
            Card { suit: CardSuit::Spade, value: CardValue::Nine },
            Card { suit: CardSuit::Spade, value: CardValue::Ten },
            Card { suit: CardSuit::Spade, value: CardValue::Jack },
            Card { suit: CardSuit::Spade, value: CardValue::Queen },
            Card { suit: CardSuit::Spade, value: CardValue::King },
            Card { suit: CardSuit::Club, value: CardValue::Ace },
            Card { suit: CardSuit::Club, value: CardValue::Two },
            Card { suit: CardSuit::Club, value: CardValue::Three },
            Card { suit: CardSuit::Club, value: CardValue::Four },
            Card { suit: CardSuit::Club, value: CardValue::Five },
            Card { suit: CardSuit::Club, value: CardValue::Six },
            Card { suit: CardSuit::Club, value: CardValue::Seven },
            Card { suit: CardSuit::Club, value: CardValue::Eight },
            Card { suit: CardSuit::Club, value: CardValue::Nine },
            Card { suit: CardSuit::Club, value: CardValue::Ten },
            Card { suit: CardSuit::Club, value: CardValue::Jack },
            Card { suit: CardSuit::Club, value: CardValue::Queen },
            Card { suit: CardSuit::Club, value: CardValue::King },
            Card { suit: CardSuit::Diamond, value: CardValue::Ace },
            Card { suit: CardSuit::Diamond, value: CardValue::Two },
            Card { suit: CardSuit::Diamond, value: CardValue::Three },
            Card { suit: CardSuit::Diamond, value: CardValue::Four },
            Card { suit: CardSuit::Diamond, value: CardValue::Five },
            Card { suit: CardSuit::Diamond, value: CardValue::Six },
            Card { suit: CardSuit::Diamond, value: CardValue::Seven },
            Card { suit: CardSuit::Diamond, value: CardValue::Eight },
            Card { suit: CardSuit::Diamond, value: CardValue::Nine },
            Card { suit: CardSuit::Diamond, value: CardValue::Ten },
            Card { suit: CardSuit::Diamond, value: CardValue::Jack },
            Card { suit: CardSuit::Diamond, value: CardValue::Queen },
            Card { suit: CardSuit::Diamond, value: CardValue::King },
        ];

        let deck = Deck::new();

        for card in cards.iter() {
            assert!(deck.contains(card));
        }
    }
}
