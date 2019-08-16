#[macro_use]
extern crate log;
extern crate env_logger;

use std::thread;
use std::net::TcpStream;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    env_logger::init();

    let stream = TcpStream::connect("localhost:2024").unwrap();

    let read_stream = stream.try_clone().expect("Failed to clone the stream for reading");

    let handle = thread::spawn(move || {
        let mut reader = BufReader::new(read_stream);
        let reader_ref = reader.by_ref();

        loop {
            let mut message = String::new();
            
            debug!("Waiting for incoming message");
            reader_ref.read_line(&mut message).unwrap();

            let size = message.trim_end().to_string().parse::<u64>().unwrap();
            debug!("Message is {} bytes long", size);

            let mut buffer = Vec::new();
            reader_ref.take(size).read_to_end(&mut buffer).unwrap();
            let message = String::from_utf8_lossy(&buffer).to_string();
            debug!("Received \"{}\" from server", message);
            println!("{}", message);
        }
    });

    handle.join().unwrap();
}
