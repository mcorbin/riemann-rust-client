use event::{Event, MsgError};
use std::time::Duration;
use std::io;
use protobuf::ProtobufError;
use event::RiemannClientError;

#[derive(Debug)]
pub enum ConnectError {
    IOError(io::Error)
}

#[derive(Debug)]
pub enum SendError {
    ProtoError(ProtobufError),
    IOError(io::Error),
    MsgError(MsgError),
    ClientError(RiemannClientError),
}

impl From<io::Error> for ConnectError {
    fn from(err: io::Error) -> ConnectError {
        ConnectError::IOError(err)
    }
}

impl From<ProtobufError> for SendError {
    fn from(err: ProtobufError) -> SendError {
        SendError::ProtoError(err)
    }
}

impl From<io::Error> for SendError {
    fn from(err: io::Error) -> SendError {
        SendError::IOError(err)
    }
}

impl From<MsgError> for SendError {
    fn from(err: MsgError) -> SendError {
        SendError::MsgError(err)
    }
}

impl From<RiemannClientError> for SendError {
    fn from(err: RiemannClientError) -> SendError {
        SendError::ClientError(err)
    }
}

pub trait Client {
    fn connect(&mut self, timeout: Duration) -> Result<(), ConnectError>;
    fn send(&mut self, events: &Vec<Event>) -> Result<(), SendError>;
}

pub trait IndexClient {
    fn query(query: &str) -> Result<Vec<Event>, bool>;
}
