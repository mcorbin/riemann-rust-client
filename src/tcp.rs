use std::io::prelude::*;
use event::{ Event, MsgError, RiemannClientError };
use codec;
use proto::proto;
use std::net::TcpStream;
use std::time::Duration;
use protobuf::{Message, parse_from_bytes};
use client::{Client, ConnectError, SendError};

#[derive(Debug)]
pub struct TcpClient {
    pub addr: String,
    pub tcp_client: Option<TcpStream>

}

impl TcpClient {
    pub fn new(host: &str, port: u32) -> TcpClient {
        TcpClient {
            addr: format!("{}:{}", host, port),
            tcp_client: None
        }
    }
}

impl Client for TcpClient {
    fn connect(&mut self, timeout: Duration) -> Result<(), ConnectError> {
        let stream = TcpStream::connect_timeout(&self.addr, timeout)?;
        stream.set_write_timeout(Some(timeout))?;
        stream.set_read_timeout(Some(timeout))?;
        self.tcp_client = Some(stream);
        Ok(())
    }

    fn send(&mut self, events: &Vec<Event>) -> Result<(), SendError> {

        if let Some(ref mut client) = self.tcp_client {
            let msg = codec::events_to_message(events);
            let size = msg.compute_size();
            let bytes = msg.write_to_bytes()?;
            client.write_all(&[((size >> 24) & 0xFF) as u8])?;
            client.write_all(&[((size >> 16) & 0xFF) as u8])?;
            client.write_all(&[((size >> 8) & 0xFF) as u8])?;
            client.write_all(&[(size & 0xFF) as u8])?;
            client.write_all(&bytes)?;
            client.flush()?;

            let mut read_size_buf: [u8; 4] = [0, 0, 0, 0];
            client.read_exact(&mut read_size_buf)?;
            let read_size: u32 = ((read_size_buf[0] as u32) << 24)
                + ((read_size_buf[1] as u32) << 16)
                + ((read_size_buf[2] as u32) << 8)
                + (read_size_buf[3] as u32);


            let mut resp = client.take(read_size as u64);
            let mut response_vec: Vec<u8> = Vec::with_capacity(read_size as usize);
            resp.read_to_end(&mut response_vec)?;

            let msg: proto::Msg = parse_from_bytes(&response_vec)?;

            if msg.has_ok() {
                let ok = msg.get_ok();
                if !ok {
                    // the Msg is on error
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
                    message: format!("Unknown error during Riemann send. Msg was {:?}", msg)
                };
                // TODO
                return Err(SendError::MsgError(msg_error))
            }
        }
        let error = RiemannClientError {
            message: format!("Riemann Client not connected ?")
        };
        Err(SendError::ClientError(error))
    }
}
