use rand::{seq::SliceRandom, thread_rng};
use std::fmt;

#[derive(PartialEq, Clone)]
enum CardSuit {
    Heart,
    Spade,
    Club,
    Diamond,
}

impl fmt::Display for CardSuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match *self {
            Self::Heart => "hearts",
            Self::Club => "clubs",
            Self::Spade => "spades",
            Self::Diamond => "diamonds",
        };

        write!(f, "{}", text)
    }
}

#[derive(PartialEq, Clone)]
pub enum CardValue {
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
    King,
}

impl fmt::Display for CardValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Ace => "ace",
            Self::Two => "two",
            Self::Three => "three",
            Self::Four => "four",
            Self::Five => "five",
            Self::Six => "six",
            Self::Seven => "seven",
            Self::Eight => "eight",
            Self::Nine => "nine",
            Self::Ten => "ten",
            Self::Jack => "jack",
            Self::Queen => "queen",
            Self::King => "king",
        };

        write!(f, "{}", text)
    }
}

#[derive(PartialEq, Clone)]
pub struct Card {
    suit: CardSuit,
    value: CardValue,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }
}

impl Card {
    pub fn value(&self) -> &CardValue {
        &self.value
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    const SIZE: u8 = 52;

    fn new() -> Deck {
        let cards = vec![
            Card {
                suit: CardSuit::Heart,
                value: CardValue::Ace,
            },
            Card {
                suit: CardSuit::Heart,
                value: CardValue::Two,
            },
            Card {
                suit: CardSuit::Heart,
                value: CardValue::Three,
            },
            Card {
                suit: CardSuit::Heart,
                value: CardValue::Four,
            },
            Card {
                suit: CardSuit::Heart,
                value: CardValue::Five,
            },
            Card {
                suit: CardSuit::Heart,
                value: CardValue::Six,
            },
            Card {
                suit: CardSuit::Heart,
                value: CardValue::Seven,
            },
            Card {
                suit: CardSuit::Heart,
                value: CardValue::Eight,
            },
            Card {
                suit: CardSuit::Heart,
                value: CardValue::Nine,
            },
            Card {
                suit: CardSuit::Heart,
                value: CardValue::Ten,
            },
            Card {
                suit: CardSuit::Heart,
                value: CardValue::Jack,
            },
            Card {
                suit: CardSuit::Heart,
                value: CardValue::Queen,
            },
            Card {
                suit: CardSuit::Heart,
                value: CardValue::King,
            },
            Card {
                suit: CardSuit::Spade,
                value: CardValue::Ace,
            },
            Card {
                suit: CardSuit::Spade,
                value: CardValue::Two,
            },
            Card {
                suit: CardSuit::Spade,
                value: CardValue::Three,
            },
            Card {
                suit: CardSuit::Spade,
                value: CardValue::Four,
            },
            Card {
                suit: CardSuit::Spade,
                value: CardValue::Five,
            },
            Card {
                suit: CardSuit::Spade,
                value: CardValue::Six,
            },
            Card {
                suit: CardSuit::Spade,
                value: CardValue::Seven,
            },
            Card {
                suit: CardSuit::Spade,
                value: CardValue::Eight,
            },
            Card {
                suit: CardSuit::Spade,
                value: CardValue::Nine,
            },
            Card {
                suit: CardSuit::Spade,
                value: CardValue::Ten,
            },
            Card {
                suit: CardSuit::Spade,
                value: CardValue::Jack,
            },
            Card {
                suit: CardSuit::Spade,
                value: CardValue::Queen,
            },
            Card {
                suit: CardSuit::Spade,
                value: CardValue::King,
            },
            Card {
                suit: CardSuit::Club,
                value: CardValue::Ace,
            },
            Card {
                suit: CardSuit::Club,
                value: CardValue::Two,
            },
            Card {
                suit: CardSuit::Club,
                value: CardValue::Three,
            },
            Card {
                suit: CardSuit::Club,
                value: CardValue::Four,
            },
            Card {
                suit: CardSuit::Club,
                value: CardValue::Five,
            },
            Card {
                suit: CardSuit::Club,
                value: CardValue::Six,
            },
            Card {
                suit: CardSuit::Club,
                value: CardValue::Seven,
            },
            Card {
                suit: CardSuit::Club,
                value: CardValue::Eight,
            },
            Card {
                suit: CardSuit::Club,
                value: CardValue::Nine,
            },
            Card {
                suit: CardSuit::Club,
                value: CardValue::Ten,
            },
            Card {
                suit: CardSuit::Club,
                value: CardValue::Jack,
            },
            Card {
                suit: CardSuit::Club,
                value: CardValue::Queen,
            },
            Card {
                suit: CardSuit::Club,
                value: CardValue::King,
            },
            Card {
                suit: CardSuit::Diamond,
                value: CardValue::Ace,
            },
            Card {
                suit: CardSuit::Diamond,
                value: CardValue::Two,
            },
            Card {
                suit: CardSuit::Diamond,
                value: CardValue::Three,
            },
            Card {
                suit: CardSuit::Diamond,
                value: CardValue::Four,
            },
            Card {
                suit: CardSuit::Diamond,
                value: CardValue::Five,
            },
            Card {
                suit: CardSuit::Diamond,
                value: CardValue::Six,
            },
            Card {
                suit: CardSuit::Diamond,
                value: CardValue::Seven,
            },
            Card {
                suit: CardSuit::Diamond,
                value: CardValue::Eight,
            },
            Card {
                suit: CardSuit::Diamond,
                value: CardValue::Nine,
            },
            Card {
                suit: CardSuit::Diamond,
                value: CardValue::Ten,
            },
            Card {
                suit: CardSuit::Diamond,
                value: CardValue::Jack,
            },
            Card {
                suit: CardSuit::Diamond,
                value: CardValue::Queen,
            },
            Card {
                suit: CardSuit::Diamond,
                value: CardValue::King,
            },
        ];

        Deck { cards }
    }

    fn cards(&mut self) -> &mut Vec<Card> {
        &mut self.cards
    }
}

pub struct Shoe {
    deck_count: u8,
    /// Minimum number of cards allowed before shoe refills itself
    minimum_cards: u16, // With maximum 255 decks, the max number this could ever hold is 3978, so 16 bits is enough
    cards: Vec<Card>,
}

impl Shoe {
    pub fn new(deck_count: u8) -> Result<Self, &'static str> {
        if deck_count < 1 {
            return Err("Deck count has to be at least 1");
        }

        let minimum_cards = Deck::SIZE as u32 * deck_count as u32 * 30 / 100;

        let mut shoe = Self {
            deck_count,
            minimum_cards: minimum_cards as u16,
            cards: Vec::with_capacity((deck_count as u16 * Deck::SIZE as u16) as usize),
        };

        shoe.refill();

        Ok(shoe)
    }

    pub fn pull_card(&mut self) -> Card{
        let card = match self.cards.pop() {
            Some(card) => card,
            None => panic!("Tried to pull card from empty shoe")
        };

        if (self.cards.len() as u16) < self.minimum_cards {
            self.refill();
        }

        card
    }

    /// Empties the shoe and refills it with new decks
    fn refill(&mut self) {
        self.cards.clear();

        for _ in 0..self.deck_count {
            let mut deck = Deck::new();
            self.cards.append(&mut deck.cards());
        }

        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    mod deck {
        use super::*;

        impl Deck {
            pub fn len(&self) -> usize {
                self.cards.len()
            }

            fn contains(&self, card: &Card) -> bool {
                self.cards.contains(card)
            }

            fn pop(&mut self) -> Option<Card> {
                self.cards.pop()
            }
        }

        #[test]
        fn deck_has_correct_number_of_cards() {
            assert_eq!(52, Deck::SIZE);
            assert_eq!(Deck::SIZE, Deck::new().len() as u8);
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
                Card {
                    suit: CardSuit::Heart,
                    value: CardValue::Ace,
                },
                Card {
                    suit: CardSuit::Heart,
                    value: CardValue::Two,
                },
                Card {
                    suit: CardSuit::Heart,
                    value: CardValue::Three,
                },
                Card {
                    suit: CardSuit::Heart,
                    value: CardValue::Four,
                },
                Card {
                    suit: CardSuit::Heart,
                    value: CardValue::Five,
                },
                Card {
                    suit: CardSuit::Heart,
                    value: CardValue::Six,
                },
                Card {
                    suit: CardSuit::Heart,
                    value: CardValue::Seven,
                },
                Card {
                    suit: CardSuit::Heart,
                    value: CardValue::Eight,
                },
                Card {
                    suit: CardSuit::Heart,
                    value: CardValue::Nine,
                },
                Card {
                    suit: CardSuit::Heart,
                    value: CardValue::Ten,
                },
                Card {
                    suit: CardSuit::Heart,
                    value: CardValue::Jack,
                },
                Card {
                    suit: CardSuit::Heart,
                    value: CardValue::Queen,
                },
                Card {
                    suit: CardSuit::Heart,
                    value: CardValue::King,
                },
                Card {
                    suit: CardSuit::Spade,
                    value: CardValue::Ace,
                },
                Card {
                    suit: CardSuit::Spade,
                    value: CardValue::Two,
                },
                Card {
                    suit: CardSuit::Spade,
                    value: CardValue::Three,
                },
                Card {
                    suit: CardSuit::Spade,
                    value: CardValue::Four,
                },
                Card {
                    suit: CardSuit::Spade,
                    value: CardValue::Five,
                },
                Card {
                    suit: CardSuit::Spade,
                    value: CardValue::Six,
                },
                Card {
                    suit: CardSuit::Spade,
                    value: CardValue::Seven,
                },
                Card {
                    suit: CardSuit::Spade,
                    value: CardValue::Eight,
                },
                Card {
                    suit: CardSuit::Spade,
                    value: CardValue::Nine,
                },
                Card {
                    suit: CardSuit::Spade,
                    value: CardValue::Ten,
                },
                Card {
                    suit: CardSuit::Spade,
                    value: CardValue::Jack,
                },
                Card {
                    suit: CardSuit::Spade,
                    value: CardValue::Queen,
                },
                Card {
                    suit: CardSuit::Spade,
                    value: CardValue::King,
                },
                Card {
                    suit: CardSuit::Club,
                    value: CardValue::Ace,
                },
                Card {
                    suit: CardSuit::Club,
                    value: CardValue::Two,
                },
                Card {
                    suit: CardSuit::Club,
                    value: CardValue::Three,
                },
                Card {
                    suit: CardSuit::Club,
                    value: CardValue::Four,
                },
                Card {
                    suit: CardSuit::Club,
                    value: CardValue::Five,
                },
                Card {
                    suit: CardSuit::Club,
                    value: CardValue::Six,
                },
                Card {
                    suit: CardSuit::Club,
                    value: CardValue::Seven,
                },
                Card {
                    suit: CardSuit::Club,
                    value: CardValue::Eight,
                },
                Card {
                    suit: CardSuit::Club,
                    value: CardValue::Nine,
                },
                Card {
                    suit: CardSuit::Club,
                    value: CardValue::Ten,
                },
                Card {
                    suit: CardSuit::Club,
                    value: CardValue::Jack,
                },
                Card {
                    suit: CardSuit::Club,
                    value: CardValue::Queen,
                },
                Card {
                    suit: CardSuit::Club,
                    value: CardValue::King,
                },
                Card {
                    suit: CardSuit::Diamond,
                    value: CardValue::Ace,
                },
                Card {
                    suit: CardSuit::Diamond,
                    value: CardValue::Two,
                },
                Card {
                    suit: CardSuit::Diamond,
                    value: CardValue::Three,
                },
                Card {
                    suit: CardSuit::Diamond,
                    value: CardValue::Four,
                },
                Card {
                    suit: CardSuit::Diamond,
                    value: CardValue::Five,
                },
                Card {
                    suit: CardSuit::Diamond,
                    value: CardValue::Six,
                },
                Card {
                    suit: CardSuit::Diamond,
                    value: CardValue::Seven,
                },
                Card {
                    suit: CardSuit::Diamond,
                    value: CardValue::Eight,
                },
                Card {
                    suit: CardSuit::Diamond,
                    value: CardValue::Nine,
                },
                Card {
                    suit: CardSuit::Diamond,
                    value: CardValue::Ten,
                },
                Card {
                    suit: CardSuit::Diamond,
                    value: CardValue::Jack,
                },
                Card {
                    suit: CardSuit::Diamond,
                    value: CardValue::Queen,
                },
                Card {
                    suit: CardSuit::Diamond,
                    value: CardValue::King,
                },
            ];

            let deck = Deck::new();

            for card in cards.iter() {
                assert!(deck.contains(card));
            }
        }
    }

    mod shoe {
        use super::*;

        #[test]
        fn invalid_deck_count() {
            assert!(Shoe::new(0).is_err());
        }

        #[test]
        fn valid_deck_sizes() {
            let shoe = create_shoe(1);
            assert_eq!(shoe.cards.len(), 52);
            assert_eq!(shoe.minimum_cards, 15);

            let shoe = create_shoe(5);
            assert_eq!(shoe.cards.len(), 260);
            assert_eq!(shoe.minimum_cards, 78);

            let shoe = create_shoe(50);
            assert_eq!(shoe.cards.len(), 2600);
            assert_eq!(shoe.minimum_cards, 780);

            let shoe = create_shoe(255);
            assert_eq!(shoe.cards.len(), 13260);
            assert_eq!(shoe.minimum_cards, 3978);
        }

        fn create_shoe(deck_count: u8) -> Shoe {
            let shoe = Shoe::new(deck_count);
            assert!(shoe.is_ok());

            shoe.unwrap()
        }

        #[test]
        fn refill() {
            let mut shoe = create_shoe(3);

            while (shoe.cards.len() as u16) > shoe.minimum_cards {
                shoe.pull_card();
            }

            assert_eq!(shoe.cards.len() as u16, shoe.minimum_cards);
            shoe.pull_card();
            assert_eq!(shoe.cards.len() as u16, shoe.deck_count as u16 * Deck::SIZE as u16);
        }
    }
}
