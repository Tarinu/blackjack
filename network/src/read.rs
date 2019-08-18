use std::{
    convert::From,
    error, fmt,
    io::{self, prelude::*},
    num::ParseIntError,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    ConnectionLost,
    InvalidBytesRead,
    IOError,
}

#[derive(Debug)]
enum Repr {
    Simple(ErrorKind),
    Extended(ExtendedError),
}

#[derive(Debug)]
struct ExtendedError {
    kind: ErrorKind,
    error: Box<dyn error::Error + Send + Sync>,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let reason = match self {
            Self::ConnectionLost => "connection with the server is lost",
            Self::InvalidBytesRead => "stream returned invalid string for message bytes length",
            Self::IOError => "IO Error",
        };

        write!(f, "{}", reason)
    }
}

#[derive(Debug)]
pub struct Error {
    repr: Repr,
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        match self.repr {
            Repr::Simple(kind) => kind,
            Repr::Extended(ref extended) => extended.kind,
        }
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.repr {
            Repr::Simple(kind) => write!(f, "Reading from stream failed: {}", kind),
            Repr::Extended(extended) => write!(
                f,
                "Reading from stream failed: {}, previous error: {}",
                extended.kind, extended.error
            ),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            repr: Repr::Simple(kind),
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Error {
        Error {
            repr: Repr::Extended(ExtendedError {
                kind: ErrorKind::InvalidBytesRead,
                error: Box::new(error),
            }),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error {
            repr: Repr::Extended(ExtendedError {
                kind: ErrorKind::IOError,
                error: Box::new(error),
            }),
        }
    }
}

pub type ReadResult = Result<String, Error>;

pub fn read(reader: &mut impl io::BufRead) -> ReadResult {
    let mut message = String::new();

    debug!("Waiting for incoming message");
    match reader.read_line(&mut message) {
        Ok(0) => return Err(Error::from(ErrorKind::ConnectionLost)),
        Ok(_) => (),
        Err(e) => return Err(Error::from(e)),
    }

    debug!("Received \"{:?}\" from server", message);
    let size = message.trim_end().to_string().parse::<u64>()?;

    debug!("Message is {} bytes long", size);

    let mut buffer = Vec::new();
    reader.take(size).read_to_end(&mut buffer)?;
    let message = String::from_utf8_lossy(&buffer).to_string();

    Ok(message)
}
