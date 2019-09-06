use std::fmt::Display;
use std::io;
use std::net::{Shutdown, TcpListener, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use network;

type ConnectionWrapper = Arc<Mutex<Connection>>;

struct Connection {
    stream: TcpStream,
    buffer: io::BufReader<TcpStream>,
}

impl Connection {
    fn new(stream: TcpStream) -> Connection {
        let clone = stream.try_clone().unwrap();
        Connection {
            stream,
            buffer: io::BufReader::new(clone),
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

/// Events that the threads can send to the server
enum ServerEvent {
    /// Store connection in the server
    Add(ConnectionWrapper),
    /// Remove the connection from server
    Drop(ConnectionWrapper)
}

pub struct Server<T> {
    address: T,
    connections: Vec<ConnectionWrapper>,
}

impl<T> Server<T>
where
    T: ToSocketAddrs,
{
    pub fn new(address: T) -> Server<T> {
        Server {
            address: address,
            connections: Vec::new(),
        }
    }

    /// Blocking call that starts the whole server
    pub fn start(&mut self) -> Result<(), io::Error> {
        let listener = TcpListener::bind(&self.address)?;
        info!(
            "Server connected to address: {}",
            listener.local_addr().unwrap()
        );

        let (sender, receiver): (mpsc::Sender<ServerEvent>, mpsc::Receiver<ServerEvent>) = mpsc::channel();
        self.listen_incoming_connections(listener, sender);

        for event in receiver.iter() {
            match event {
                ServerEvent::Add(connection) => self.push_connection(connection),
                ServerEvent::Drop(connection) => self.close_connection(connection)
            }
        }

        Ok(())
    }

    fn listen_incoming_connections(&self, listener: TcpListener, sender: mpsc::Sender<ServerEvent>) {
        thread::spawn(move || {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                info!("New incoming connection: {}", stream.peer_addr().unwrap());

                let connection = Arc::new(Mutex::new(Connection::new(stream)));
                let thread_connection = connection.clone();
                sender.send(ServerEvent::Add(connection)).unwrap();

                let sender = sender.clone();

                thread::spawn(move || {
                    while let Ok(connection) = thread_connection.lock() {
                        let mut connection = connection;
                        match connection.read() {
                            Ok(message) => match message {
                                network::Message::Exit => {
                                    sender.send(ServerEvent::Drop(thread_connection.clone())).unwrap();
                                    break;
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
        self.connections.retain(|conn| {
            !Arc::ptr_eq(&connection, conn)
        });
    }
}
