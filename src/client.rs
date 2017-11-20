use event::{Event, Msg, Query};
use std::time::Duration;
use std::io;
use protobuf::ProtobufError;
use event::RiemannClientError;

#[derive(Debug)]
pub enum ConnectError {
    IOError(io::Error),
}

#[derive(Debug)]
pub enum SendError {
    ProtoError(ProtobufError),
    IOError(io::Error),
    MsgError(Msg),
    ClientError(RiemannClientError),
}

impl From<io::Error> for ConnectError {
    fn from(err: io::Error) -> ConnectError {
        ConnectError::IOError(err)
    }
}

impl From<Msg> for SendError {
    fn from(err: Msg) -> SendError {
        SendError::MsgError(err)
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

impl From<RiemannClientError> for SendError {
    fn from(err: RiemannClientError) -> SendError {
        SendError::ClientError(err)
    }
}

pub trait Client {
    fn connect(&mut self, timeout: Duration) -> Result<(), ConnectError>;
    fn send(&mut self, events: &Vec<Event>) -> Result<(), SendError>;
    fn close(&mut self) -> ();
}

pub trait Index {
    fn query(&mut self, query: Query) -> Result<Msg, SendError>;
}
