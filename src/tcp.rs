use std::io::prelude::*;
use event::{ Event, MsgError };
use codec;
use proto::proto;
use std::net::TcpStream;
use std::time::Duration;
use protobuf::{Message, parse_from_bytes};
use client::{Client, ConnectError, SendError};

#[derive(Debug)]
pub struct TcpClient {
    pub host: String,
    pub port: i32,
    pub tcpClient: Option<TcpStream>

}

impl TcpClient {
    pub fn new(host: &str, port: i32) -> TcpClient {
        TcpClient {
            host: host.to_owned(),
            port: port,
            tcpClient: None
        }
    }
}

impl Client for TcpClient {
    fn connect(&mut self, timeout: Duration) -> Result<bool, ConnectError> {
        let stream = TcpStream::connect("127.0.0.1:34254")?;
        stream.set_write_timeout(Some(timeout))?;
        stream.set_read_timeout(Some(timeout))?;
        self.tcpClient = Some(stream);
        Ok(true)
    }

    fn send(&mut self, event: &Event) -> Result<(), SendError> {
        let proto_event = codec::event_to_proto(event);

        if let Some(ref mut client) = self.tcpClient {
            let size = proto_event.compute_size();
            let bytes = proto_event.write_to_bytes()?;
            client.write(&[(size >> 24 & 0xFFFFFF) as u8])?;
            client.write(&[(size >> 16 & 0xFFFFFF) as u8])?;
            client.write(&[(size >> 8 & 0xFFFFFF) as u8])?;
            client.write(&[(size & 0xFFFFFF) as u8])?;
            client.write(&[(size & 0xFFFFFF) as u8])?;
            client.write_all(&bytes)?;

            let mut read_size_buf: [u8; 4] = [0, 0, 0, 0];
            client.read_exact(&mut read_size_buf);
            let read_size: u32 = ((read_size_buf[0] as u32) << 24)
                + ((read_size_buf[1] as u32) << 16)
                + ((read_size_buf[2] as u32) << 8)
                + (read_size_buf[3] as u32);

            let mut response_vec: Vec<u8> = Vec::with_capacity(read_size as usize);
            client.read_exact(&mut response_vec);
            let msg: proto::Msg = parse_from_bytes(&response_vec)?;

            if msg.has_ok() {
                let ok = msg.get_ok();
                if !ok {
                    let msg_error = MsgError { message: msg.get_error().to_owned() };
                    // TODO
                    return Err(SendError::MsgError(msg_error))
                }
                else {
                    return Ok(())
                }
            }
            else {
                let msg_error = MsgError {
                    message: "Unknown error during Riemann send".to_owned()
                };
                // TODO
                return Err(SendError::MsgError(msg_error))
            }
        }
        // TODO
        Ok(())
    }

    fn send_events(events: &Vec<Event>) -> Result<bool, bool> {
        Ok(true)
    }

    fn close() -> Result<bool, bool> {
        Ok(true)
    }
}
