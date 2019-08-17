#[macro_use]
extern crate log;

use std::{
    error::Error,
    fmt,
    io::{self, prelude::*, Write},
    net::TcpStream,
    str::FromStr,
};

#[derive(Debug)]
pub struct InvalidMessageError {
    message: String,
}

impl Error for InvalidMessageError {}

impl fmt::Display for InvalidMessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid message \"{}\"", self.message)
    }
}

#[derive(PartialEq, Debug)]
pub enum Message {
    Exit,
}

impl FromStr for Message {
    type Err = InvalidMessageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.trim() {
            "exit" => Ok(Self::Exit),
            _ => Err(InvalidMessageError { message: s }),
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            Self::Exit => "exit",
        };

        write!(f, "{}", text)
    }
}

pub fn send(mut stream: &TcpStream, message: impl fmt::Display) -> Result<(), io::Error> {
    let string = message.to_string();
    let bytes = string.as_bytes().len();
    debug!("Writing {} bytes to stream", bytes);
    stream.write_all(format!("{}\r\n{}", bytes, string).as_bytes())?;

    Ok(())
}

pub fn read(reader: &mut impl io::BufRead) -> Result<String, Box<dyn Error>> {
    let mut message = String::new();

    debug!("Waiting for incoming message");
    reader.read_line(&mut message)?;

    let size = message.trim_end().to_string().parse::<u64>()?;
    debug!("Message is {} bytes long", size);

    let mut buffer = Vec::new();
    reader.take(size).read_to_end(&mut buffer)?;
    let message = String::from_utf8_lossy(&buffer).to_string();
    debug!("Received \"{}\" from server", message);

    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod message {
        use super::Message;
        #[test]
        fn message_valid_parsing() {
            assert_eq!(Message::Exit, "exit".parse().unwrap());
            assert_eq!(Message::Exit, "EXIT".parse().unwrap());
            assert_eq!(Message::Exit, "eXIt".parse().unwrap());
            assert_eq!(Message::Exit, "  exit  ".parse().unwrap());
        }

        #[test]
        fn message_parse_types() {
            assert_eq!(Message::Exit, "exit".parse().unwrap());
        }
    }
}
