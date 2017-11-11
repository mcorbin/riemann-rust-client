use event::{Event, MsgError};
use std::time::Duration;
use std::io;
use protobuf::ProtobufError;

#[derive(Debug)]
pub enum ConnectError {
    IOError(io::Error)
}

#[derive(Debug)]
pub enum SendError {
    ProtoError(ProtobufError),
    IOError(io::Error),
    MsgError(MsgError)
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

pub trait Client {
    fn connect(&mut self, timeout: Duration) -> Result<bool, ConnectError>;
    fn send(&mut self, event: &Event) -> Result<(), SendError>;
    fn send_events(events: &Vec<Event>) -> Result<bool, bool>;
    fn close() -> Result<bool, bool>;
}

pub trait IndexClient {
    fn query(query: &str) -> Result<Vec<Event>, bool>;
}
