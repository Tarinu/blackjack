#[macro_use]
extern crate log;
extern crate env_logger;

use std::net;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    env_logger::init();

    let stream = net::TcpStream::connect("localhost:2024").unwrap();

    let mut reader = BufReader::new(&stream);
    let mut message = String::new();
    
    debug!("Waiting for incoming message");
    reader.read_line(&mut message).unwrap();
    debug!("{}", message);
    let size = message.trim_end().to_string().parse::<u64>().unwrap();
    debug!("Message is {} bytes long", size);
    let mut buffer = Vec::new();
    reader.take(size).read_to_end(&mut buffer).unwrap();
    let message = String::from_utf8(buffer).unwrap();
    debug!("Received {} from server", message);
    println!("{}", message);
}
