mod blackjack;
mod deck;
mod server;

fn main() {
    env_logger::init();
    let mut server = server::Server::new("localhost:2024");
    server.start().unwrap();
}
