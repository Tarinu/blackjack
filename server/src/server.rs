use std::fmt::Display;
use std::io;
use std::net::{Shutdown, TcpListener, TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::thread;

use network;

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

pub struct Server<T> {
    address: T,
    connections: Vec<Arc<Mutex<Connection>>>,
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

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            info!("New incoming connection: {}", stream.peer_addr().unwrap());

            let connection = Arc::new(Mutex::new(Connection::new(stream)));
            let thread_connection = connection.clone();
            self.connections.push(connection);

            thread::spawn(move || {
                while let Ok(connection) = thread_connection.lock() {
                    let mut connection = connection;
                    match connection.read() {
                        Ok(message) => match message {
                            network::Message::Exit => {
                                connection.close();
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

        Ok(())
    }
}
