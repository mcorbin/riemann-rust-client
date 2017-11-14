use std::io::{Write, Read};
use event::{ Event, MsgError, RiemannClientError };
use codec;
use proto::proto;
use std::net::{TcpStream, SocketAddr, AddrParseError};
use std::time::Duration;
use protobuf::{Message, parse_from_bytes};
use client::{Client, ConnectError, SendError};

trait ReadWrite : Read + Write {}

impl<T: Read + Write> ReadWrite for T {}

pub struct TcpClient {
    pub host: String,
    pub port: u32,
    pub addr: SocketAddr,
    pub stream: Option<TcpStream>,
}

impl TcpClient {
    pub fn new(host: &str, port: u32)
               -> Result<TcpClient, AddrParseError> {
        let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
        Ok(TcpClient {
            addr: addr,
            host: host.to_owned(),
            port: port,
            stream: None
        })
    }
}

impl Client for TcpClient {
    fn connect(&mut self, timeout: Duration) -> Result<(), ConnectError> {
        let stream = TcpStream::connect_timeout(&self.addr, timeout)?;
        stream.set_write_timeout(Some(timeout))?;
        stream.set_read_timeout(Some(timeout))?;
        self.stream = Some(stream);

        Ok(())
    }

    fn send(&mut self, events: &Vec<Event>) -> Result<(), SendError> {
        if let Some(ref mut client) = self.stream {

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
