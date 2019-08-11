#[macro_use]
extern crate log;
extern crate env_logger;

mod deck;
mod server;

use deck::Deck;

fn main() {
    env_logger::init();
    let mut _deck = Deck::new();
    let mut server = server::Server::new("localhost:2024");
    server.start().unwrap();
}
