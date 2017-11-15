use std::net::UdpSocket;
use client::{Client, ConnectError, SendError};
use std::time::Duration;
use event::Event;
use event::{RiemannClientError};
use std::net::SocketAddr;
use protobuf::{Message};
use codec;

#[derive(Debug)]
pub struct UdpClient {
    pub bind_addr: SocketAddr,
    pub addr: SocketAddr,
    pub socket: Option<UdpSocket>
}

const MAX_UDP_SIZE: u32 = 16384;

impl UdpClient {
    pub fn new(addr: SocketAddr,
               bind_addr: SocketAddr) -> UdpClient {
        UdpClient {
            bind_addr: bind_addr,
            addr: addr,
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
                let error = RiemannClientError {
                    message: format!("Datagram size: {}, max size: {}", size, MAX_UDP_SIZE)
                };
                return Err(SendError::ClientError(error));
            }
            let bytes = msg.write_to_bytes()?;
            socket.send_to(&bytes, &self.addr)?;
            return Ok(());
        }
        let error = RiemannClientError {
            message: format!("Riemann Client not connected ?")
        };
        Err(SendError::ClientError(error))
    }

    fn close(&mut self) {
        self.socket = None;
    }

}
