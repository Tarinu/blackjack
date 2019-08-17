#[macro_use]
extern crate log;
extern crate env_logger;

use network;
use std::io::prelude::*;
use std::io::{stdin, BufReader};
use std::net::TcpStream;
use std::thread;

fn main() {
    env_logger::init();

    let stream = TcpStream::connect("localhost:2024").unwrap();

    let read_stream = stream
        .try_clone()
        .expect("Failed to clone the stream for reading");

    thread::spawn(move || {
        let mut reader = BufReader::new(read_stream);
        let reader_ref = reader.by_ref();

        loop {
            let message = network::read(reader_ref);
            println!("{}", message.unwrap());
        }
    });

    for line in stdin().lock().lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(e) => {
                warn!("{}", e);
                println!("Invalid command");
            }
        }
    }
}
