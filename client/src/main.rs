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
            match network::read::read(reader_ref) {
                Ok(message) => println!("{}", message),
                Err(e) => {
                    if e.kind() == network::read::ErrorKind::ConnectionLost {
                        println!("Connection to the server lost");
                        break;
                    } else {
                        warn!("{}", e);
                    }
                }
            }
        }

        debug!("Reader thread reached its end");
    });

    for line in stdin().lock().lines() {
        match line {
            Ok(line) => match line.parse::<network::Message>() {
                Ok(message) => {
                    if let Err(e) = network::send(&stream, message) {
                        error!("{}", e);
                        println!("Failed to send the command to the server");
                    }
                }
                Err(e) => {
                    warn!("{}", e);
                    println!("Invalid command");
                }
            },
            Err(e) => {
                warn!("{}", e);
                println!("Invalid command");
            }
        }
    }
}
