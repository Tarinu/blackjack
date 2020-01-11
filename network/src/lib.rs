#[macro_use]
extern crate log;

pub mod read;

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

pub type MessageResult = Result<Message, InvalidMessageError>;

#[derive(PartialEq, Debug)]
pub enum Message {
    Exit,
    Login(String),
    Balance,
    Deposit(u32),
    Start(u32),
    Hit,
    Stand
}

impl FromStr for Message {
    type Err = InvalidMessageError;

    fn from_str(s: &str) -> MessageResult {
        let s = s.trim();
        let mut split = s.split_whitespace();
        match split.next() {
            Some(keyword) => match keyword.to_lowercase().as_ref() {
                "exit" => Ok(Self::Exit),
                "login" => Ok(Self::Login(split.collect::<Vec<&str>>().join(" "))),
                "balance" => Ok(Self::Balance),
                "deposit" => {
                    match split.next() {
                        Some(amount) => {
                            match amount.to_string().parse::<u32>() {
                                Ok(amount) => {
                                    if amount <= 0 {
                                        return Err(InvalidMessageError { message: String::from("Deposit amount has to be a positive number") });
                                    }

                                    Ok(Self::Deposit(amount))
                                },
                                Err(e) => {
                                    warn!("{}", e);
                                    Err(InvalidMessageError { message: String::from("Unable to parse input to integer") })
                                }
                            }
                        },
                        None => Err(InvalidMessageError { message: String::from("Amount missing") })
                    }
                },
                "start" => {
                    match split.next() {
                        Some(amount) => {
                            match amount.to_string().parse::<u32>() {
                                Ok(amount) => {
                                    if amount <= 0 {
                                        return Err(InvalidMessageError { message: String::from("Bet amount has to be a positive number") });
                                    }

                                    Ok(Self::Start(amount))
                                },
                                Err(e) => {
                                    warn!("{}", e);
                                    Err(InvalidMessageError { message: String::from("Unable to parse input to integer") })
                                }
                            }
                        },
                        None => Err(InvalidMessageError { message: String::from("Amount missing") })
                    }
                },
                "hit" => Ok(Self::Hit),
                "stand" => Ok(Self::Stand),
                _ => Err(InvalidMessageError { message: s.to_string() }),
            },
            None => Err(InvalidMessageError { message: String::from("Keyword missing") })
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            Self::Exit => String::from("exit"),
            Self::Login(username) => format!("login {}", username),
            Self::Balance => String::from("balance"),
            Self::Deposit(amount) => format!("deposit {}", amount),
            Self::Start(amount) => format!("start {}", amount),
            Self::Hit => String::from("hit"),
            Self::Stand => String::from("stand")
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

#[cfg(test)]
mod tests {
    use super::*;

    mod message {
        use super::Message;
        #[test]
        fn valid_parsing() {
            assert_eq!(Message::Exit, "exit".parse().unwrap());
            assert_eq!(Message::Exit, "EXIT".parse().unwrap());
            assert_eq!(Message::Exit, "eXIt".parse().unwrap());
            assert_eq!(Message::Exit, "  exit  ".parse().unwrap());
        }

        #[test]
        fn parse_all_types() {
            assert_eq!(Message::Exit, "exit".parse().unwrap());
            assert_eq!(Message::Login("fooBAR".to_string()), "login fooBAR".parse().unwrap());
            assert_eq!(Message::Balance, "balance".parse().unwrap());
            assert_eq!(Message::Deposit(50), "deposit 50".parse().unwrap());
            assert_eq!(Message::Start(25), "start 25".parse().unwrap());
            assert_eq!(Message::Hit, "hit".parse().unwrap());
            assert_eq!(Message::Stand, "stand".parse().unwrap());
        }
    }
}
