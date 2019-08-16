use std::io;
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::Write;
use std::fmt::Display;
use std::net::{
    TcpListener,
    TcpStream,
    ToSocketAddrs,
    Shutdown
};

struct Connection {
    stream: TcpStream
}

impl Connection {
    fn send(&mut self, message: impl Display) {
        let string = message.to_string();
        let bytes = string.as_bytes().len();
        debug!("Writing {} bytes to stream", bytes);
        self.stream.write_all(format!("{}\r\n{}", bytes, string).as_bytes()).unwrap()
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        info!("Connection is being dropped.");
        self.stream.write_all("Server is shutting down.".as_bytes()).unwrap();
        self.stream.shutdown(Shutdown::Both).unwrap();
    }
}

pub struct Server<T> {
    address: T,
    connections: Vec<Arc<Mutex<Connection>>>
}

impl<T> Server<T> where T: ToSocketAddrs {
    pub fn new(address: T) -> Server<T> {
        Server {
            address: address,
            connections: Vec::new()
        }
    }

    /// Blocking call that starts the whole server
    pub fn start(&mut self) -> Result<(), io::Error> {
        let listener = TcpListener::bind(&self.address)?;
        info!("Server connected to address: {}", listener.local_addr().unwrap());

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            info!("New incoming connection: {}", stream.peer_addr().unwrap());

            let connection = Arc::new(Mutex::new(Connection {stream: stream}));
            let thread_connection = connection.clone();
            self.connections.push(connection);

            thread::spawn(move || {
                let mut connection = thread_connection.lock().unwrap();
                connection.send("Connected");
            });
        }

        Ok(())
    }
}
