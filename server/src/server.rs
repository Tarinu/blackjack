use log::*;
use std::collections::HashMap;
use std::fmt::Display;
use std::io;
use std::net::{Shutdown, TcpListener, TcpStream, ToSocketAddrs};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::blackjack;
use crate::blackjack::Blackjack;
use network;

type BlackjackWrapper = Arc<Mutex<Blackjack>>;

struct Connection {
    stream: TcpStream,
    buffer: io::BufReader<TcpStream>,
    blackjack: Option<BlackjackWrapper>,
}

impl Connection {
    fn new(stream: TcpStream) -> Connection {
        let clone = stream.try_clone().unwrap();
        Connection {
            stream,
            buffer: io::BufReader::new(clone),
            blackjack: None,
        }
    }

    fn send(&mut self, message: impl Display) {
        network::send(&self.stream, message).unwrap();
    }

    fn read(&mut self) -> network::MessageResult {
        network::read::read(&mut self.buffer).unwrap().parse()
    }

    fn close(&mut self) {
        info!(
            "Connection {} is closing.",
            self.stream.peer_addr().unwrap()
        );
        self.stream.shutdown(Shutdown::Both).unwrap();
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        self.close();
    }
}

type ConnectionWrapper = Arc<Mutex<Connection>>;
type LoginResult = Result<BlackjackWrapper, String>;

/// Events that the threads can send to the server
enum ServerEvent {
    /// Store connection in the server
    Add(ConnectionWrapper),
    /// Remove the connection from server
    Drop(ConnectionWrapper),
    Login(String, mpsc::Sender<LoginResult>)
}

pub struct Server<T> {
    address: T,
    connections: Vec<ConnectionWrapper>,
    blackjack_instances: HashMap<String, BlackjackWrapper>,
}

impl<T> Server<T>
where
    T: ToSocketAddrs,
{
    pub fn new(address: T) -> Server<T> {
        Server {
            address: address,
            connections: Vec::new(),
            blackjack_instances: HashMap::new()
        }
    }

    /// Blocking call that starts the whole server
    pub fn start(&mut self) -> Result<(), io::Error> {
        let listener = TcpListener::bind(&self.address)?;
        info!(
            "Server connected to address: {}",
            listener.local_addr().unwrap()
        );

        let (sender, receiver): (mpsc::Sender<ServerEvent>, mpsc::Receiver<ServerEvent>) =
            mpsc::channel();
        self.listen_incoming_connections(listener, sender);

        for event in receiver.iter() {
            match event {
                ServerEvent::Add(connection) => self.push_connection(connection),
                ServerEvent::Drop(connection) => self.close_connection(connection),
                ServerEvent::Login(username, login_sender) => {
                    let username = username.to_lowercase();
                    
                    let instance = match self.blackjack_instances.get(&username) {
                        Some(instance) => instance.clone(),
                        None => {
                            let instance = Arc::new(Mutex::new(Blackjack::new()));
                            self.blackjack_instances.insert(username, instance.clone());
                            instance
                        }
                    };

                    login_sender.send(Ok(instance)).unwrap();
                }
            }
        }

        Ok(())
    }

    fn listen_incoming_connections(
        &self,
        listener: TcpListener,
        sender: mpsc::Sender<ServerEvent>,
    ) {
        thread::spawn(move || {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                info!("New incoming connection: {}", stream.peer_addr().unwrap());

                let mut connection = Connection::new(stream);
                connection.send("Type \"exit\" any time to stop the process. Type \"login <name>\" to start a new game (or if such a name already exists, resume the game).");
                let connection = Arc::new(Mutex::new(connection));
                let thread_connection = connection.clone();
                sender.send(ServerEvent::Add(connection)).unwrap();

                let sender = sender.clone();

                thread::spawn(move || {
                    while let Ok(connection) = thread_connection.lock() {
                        let mut connection = connection;
                        match connection.read() {
                            Ok(message) => match message {
                                network::Message::Exit => {
                                    sender
                                        .send(ServerEvent::Drop(thread_connection.clone()))
                                        .unwrap();
                                    break;
                                },
                                network::Message::Login(login) => {
                                    let (login_sender, login_receiver) = mpsc::channel();
                                    sender.send(ServerEvent::Login(login, login_sender)).unwrap();
                                    match login_receiver.recv() {
                                        Ok(result) => match result {
                                            Ok(blackjack) => {
                                                connection.blackjack = Some(blackjack.clone());
                                                let blackjack = blackjack.lock().unwrap();
                                                connection.send(format!("Successfully logged in. Current balance: {}. Type \"deposit <int>\" to add to your balance or \"start <int>\" to start a new game", blackjack.balance()));
                                            },
                                            Err(e) => {
                                                warn!("{}", e);
                                                connection.send(e);
                                            }
                                        },
                                        Err(e) =>  {
                                            panic!(e)
                                        }
                                    }
                                },
                                network::Message::Balance => {
                                    let message = match &connection.blackjack {
                                        Some(blackjack) => blackjack.lock().unwrap().balance().to_string(),
                                        None => "Not logged in".to_string()
                                    };

                                    connection.send(message);
                                }
                                network::Message::Deposit(amount) => {
                                    // message variable is necessary since it's not possible to send message to the connection in the match blocks
                                    // &connection.blackjack accesses the connection in an immutable context so it's not possible to do any mutable operations on it
                                    let message = match &connection.blackjack {
                                        Some(blackjack) => {
                                            let mut blackjack = blackjack.lock().unwrap();
                                            match blackjack.deposit(amount) {
                                                Ok(_) => format!("{} deposited to the account", amount),
                                                Err(e) => {
                                                    warn!("{}", e);
                                                    e.to_string()
                                                }
                                            }
                                        },
                                        None => "Not logged in".to_string()
                                    };

                                    connection.send(message);
                                },
                                network::Message::Start(amount) => {
                                    let message = match &connection.blackjack {
                                        Some(blackjack) => {
                                            let mut blackjack = blackjack.lock().unwrap();
                                            match blackjack.start(amount) {
                                                Ok(_) => format!("Dealer Hand: {}\r\nPlayer Hand: {}\r\nType \"hit\" or \"stand\"", blackjack.dealer_total(), blackjack.player_total()),
                                                Err(e) => {
                                                    warn!("{}", e);
                                                    e.to_string()
                                                }
                                            }
                                        },
                                        None => "Not logged in".to_string()
                                    };

                                    connection.send(message);
                                },
                                network::Message::Hit => {
                                    let message = match &connection.blackjack {
                                        Some(blackjack) => {
                                            let mut blackjack = blackjack.lock().unwrap();
                                            match blackjack.hit() {
                                                Ok(result) => match result {
                                                    blackjack::HitResult::Continue => format!("Dealer Hand: {}\r\nPlayer Hand: {}\r\nType \"hit\" or \"stand\"", blackjack.dealer_total(), blackjack.player_total()),
                                                    blackjack::HitResult::Bust => format!("Bust! Dealer Hand: {}\r\nPlayer Hand: {}\r\nType \"start <int>\" to start a new game", blackjack.dealer_total(), blackjack.player_total()),
                                                },
                                                Err(e) => {
                                                    warn!("{}", e);
                                                    e.to_string()
                                                }
                                            }
                                        },
                                        None => "Not logged in".to_string()
                                    };

                                    connection.send(message);
                                },
                                network::Message::Stand => {
                                    let message = match &connection.blackjack {
                                        Some(blackjack) => {
                                            let mut blackjack = blackjack.lock().unwrap();
                                            match blackjack.stand() {
                                                Ok(winner) => format!("{}! Dealer Hand: {}\r\nPlayer Hand: {}\r\nType \"start <int>\" to start a new game", winner, blackjack.dealer_total(), blackjack.player_total()),
                                                Err(e) => {
                                                    warn!("{}", e);
                                                    e.to_string()
                                                }
                                            }
                                        },
                                        None => "Not logged in".to_string()
                                    };

                                    connection.send(message);
                                }
                            },
                            Err(e) => {
                                warn!("{}", e);
                                connection.send("Server couldn't understand the command");
                            }
                        }
                    }
                });
            }
        });
    }

    fn push_connection(&mut self, connection: ConnectionWrapper) {
        self.connections.push(connection);
    }

    fn close_connection(&mut self, connection: ConnectionWrapper) {
        // No need to manually close the connection since it gets dropped anyway once all arc pointers are gone
        self.connections
            .retain(|conn| !Arc::ptr_eq(&connection, conn));
    }
}
