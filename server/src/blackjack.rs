use crate::deck;
use std::fmt;

#[derive(PartialEq)]
enum State {
    Ongoing,
    Finished
}

pub enum Winner {
    Player,
    Dealer,
    Draw
}

pub enum HitResult {
    Continue,
    Bust
}

impl fmt::Display for Winner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            Self::Player => "Player Won",
            Self::Dealer => "Dealer Won",
            Self::Draw => "Draw",
        };

        write!(f, "{}", text)
    }
}

type Hand = Vec<deck::Card>;

pub struct Blackjack {
    shoe: deck::Shoe,
    state: State,
    dealer_hand: Hand,
    player_hand: Hand,
    bet: u32,
    balance: u32,
}

impl deck::CardValue {
    fn value(&self) -> u8 {
        match self {
            Self::Ace => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten => 10,
            Self::Jack => 10,
            Self::Queen => 10,
            Self::King => 10
        }
    }
}

impl Blackjack {
    pub fn new() -> Self {
        Self {
            shoe: deck::Shoe::new(3).unwrap(),
            state: State::Finished,
            dealer_hand: Vec::new(),
            player_hand: Vec::new(),
            bet: 0,
            balance: 0
        }
    }

    pub fn balance(&self) -> u32 {
        self.balance
    }

    pub fn deposit(&mut self, deposit: u32) -> Result<(), String> {
        if deposit == 0 {
            return Err("Deposit has to be bigger than 0".to_string());
        }

        self.balance += deposit;

        Ok(())
    }

    pub fn start(&mut self, bet: u32) -> Result<(), String> {
        if self.state == State::Ongoing {
            return Err("This instance is already running".to_string());
        }

        if bet > self.balance {
            return Err(format!("Not enough balance ({}) to accept the bet ({})", self.balance, bet));
        }

        self.bet = bet;

        self.dealer_hand.clear();
        self.player_hand.clear();

        self.state = State::Ongoing;

        for _ in 0..2 {
            Self::add_card(&mut self.shoe, &mut self.dealer_hand);
            Self::add_card(&mut self.shoe, &mut self.player_hand);
        }

        Ok(())
    }

    pub fn hit(&mut self) -> Result<HitResult, String> {
        if self.state != State::Ongoing {
            return Err("Instance not ongoing".to_string());
        }

        Self::add_card(&mut self.shoe, &mut self.player_hand);

        if Self::hand_value(&self.player_hand) > 21 {
            self.balance -= self.bet;
            self.state = State::Finished;
            return Ok(HitResult::Bust);
        }

        Ok(HitResult::Continue)
    }

    // Can't use &mut self here since it gets called from other mutable self contexts, and you cant borrow something as mutable multiple times
    fn add_card(shoe: &mut deck::Shoe, hand: &mut Hand) -> deck::Card {
        let card = shoe.pull_card();

        hand.push(card.clone());

        card
    }

    pub fn stand(&mut self) -> Result<Winner, String> {
        if self.state != State::Ongoing {
            return Err("Instance not ongoing".to_string());
        }

        let player_value = Self::hand_value(&self.player_hand);
        let mut dealer_value = Self::hand_value(&self.dealer_hand);

        while dealer_value < player_value && dealer_value < 16 {
            let card = Self::add_card(&mut self.shoe, &mut self.dealer_hand);
            dealer_value += card.value().value();
        }

        // There's no need to check if player's hand is bust since stand can only be called when it's not
        let winner = if player_value == 21 || Self::is_bust(&self.dealer_hand) || player_value > dealer_value {
            self.balance += self.bet;
            Winner::Player
        } else if player_value != dealer_value {
            self.balance -= self.bet;
            Winner::Dealer
        } else {
            Winner::Draw
        };

        self.state = State::Finished;

        Ok(winner)
    }

    pub fn player_total(&self) -> u8 {
        Self::hand_value(&self.player_hand)
    }

    pub fn dealer_total(&self) -> u8 {
        Self::hand_value(&self.dealer_hand)
    }

    fn hand_value(hand: &Hand) -> u8 {
        hand.iter().map(|card| card.value().value()).sum()
    }

    fn is_bust(hand: &Hand) -> bool {
        let total = Self::hand_value(&hand);

        total > 21
    }
}
