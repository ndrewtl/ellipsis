// Standard error for ellipsis


// For errors
use std::io;
use git2;
use serde_json;

use std::{error,fmt,convert};


#[derive(Debug)]
pub enum ErrorKind {
    IO,
    Git,
    JSON
}

// Implement the default formatter for ErrorKind
impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error/{:?}", self)
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String
}

impl Error {
    pub fn new(kind: ErrorKind, message : String) -> Self {
        Self {
            kind,
            message
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

}

impl error::Error for Error {
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error/{:?}: {}", self.kind, self.message)
    }
}

impl convert::From<io::Error> for Error {
    fn from(e : io::Error) -> Self {
        Error {
            kind: ErrorKind::IO,
            message: e.to_string()
        }
    }
}

impl convert::From<git2::Error> for Error {
    fn from(e : git2::Error) -> Self {
        Error {
            kind: ErrorKind::Git,
            message: e.to_string()
        }
    }
}

impl convert::From<serde_json::Error> for Error {
    fn from (e : serde_json::Error) -> Self {
        Error {
            kind: ErrorKind::JSON,
            message: e.to_string()
        }
    }
}
