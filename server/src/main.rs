mod deck;

use deck::Deck;

fn main() {
    let mut deck = Deck::new();

    println!("{}", deck.len());
    while let Some(card) = deck.pop() {
        println!("{}", card);
    }
    println!("{}", deck.len());
}
