mod deck;

use deck::Deck;

fn main() {
    let mut deck = Deck::new();
    let mut deck2 = Deck::new();

    println!("{}", deck.len());
    while let Some(card) = deck.pop() {
        println!("{}", card);
    }
    println!("{}", deck.len());

    println!("{}", deck2.len());
    while let Some(card) = deck2.pop() {
        println!("{}", card);
    }
    println!("{}", deck2.len());
}
