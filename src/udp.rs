use std::net::UdpSocket;
use client::{Client, ConnectError, SendError};
use std::time::Duration;
use event::Event;
use event::MsgError;
use protobuf::{Message, parse_from_bytes};
use codec;

#[derive(Debug)]
pub struct UdpClient {
    pub bind_addr: String,
    pub addr: String,
    pub socket: Option<UdpSocket>
}

const MAX_UDP_SIZE: u32 = 16384;

impl UdpClient {
    pub fn new(host: &str,
               port: u32,
               bind_ip: &str,
               bind_port: Option<u32>) -> UdpClient {
        UdpClient {
            bind_addr: format!("{}:{}", bind_ip, bind_port.unwrap_or(0)),
            addr: format!("{}:{}", host, port),
            socket: None
        }
    }
}


impl Client for UdpClient {

    fn connect(&mut self, timeout: Duration) -> Result<(), ConnectError> {
        let socket = UdpSocket::bind(&self.bind_addr)?;
        socket.set_write_timeout(Some(timeout))?;
        socket.set_read_timeout(Some(timeout))?;
        self.socket = Some(socket);
        Ok(())
    }

    fn send(&mut self, events: &Vec<Event>) -> Result<(), SendError> {
        if let Some(ref mut socket) = self.socket {
            let msg = codec::events_to_message(events);
            let size = msg.compute_size();
            if size > MAX_UDP_SIZE {
                // TODO error
            }
            let bytes = msg.write_to_bytes()?;
            socket.send_to(&bytes, &self.addr)?;
            return Ok(());
        }
        let msg_error = MsgError {
            message: format!("Riemann Client not connected ?")
        };
        Err(SendError::MsgError(msg_error))
    }

}
