//! A composite error type for errors that can occur while interacting with Appveyor.

// errors
// use std::{self, fmt};
use std;
use hyper;
use serde_json;


#[derive(Debug)]
pub enum Error {
    ///The response from Twitter gave a response code that indicated an error. The enclosed value
    ///was the response code.
    BadStatus(hyper::status::StatusCode),
    ///The web request experienced an error. The enclosed value was returned from hyper.
    NetError(hyper::error::Error),
    ///An error was experienced while processing the response stream. The enclosed value was
    ///returned from libstd.
    IOError(std::io::Error),
    // Failed to parse data as JSON
    ParseError(serde_json::error::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::BadStatus(ref val) => write!(f, "Error status received: {}", val),
            Error::NetError(ref err) => write!(f, "Network error: {}", err),
            Error::IOError(ref err) => write!(f, "IO error: {}", err),
            Error::ParseError(ref err) => write!(f, "Parsing error: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadStatus(_) => "Response included error code",
            Error::NetError(ref err) => err.description(),
            Error::IOError(ref err) => err.description(),
            Error::ParseError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::NetError(ref err) => Some(err),
            Error::IOError(ref err) => Some(err),
            Error::ParseError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Error {
        Error::NetError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOError(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::ParseError(err)
    }
}
