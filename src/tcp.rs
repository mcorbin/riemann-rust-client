use std::io;
use tokio_core::net::TcpStream;
use tokio_proto;
use tokio_core::reactor::{Handle, Core};
use tokio_proto::pipeline::{ClientService};
use tokio_service::{Service};
use std::net::SocketAddr;
use futures::{Future};
use client;

// client

pub struct TcpClient {
    inner: ClientService<TcpStream, client::RiemannProto>,
}

impl TcpClient {
    /// Establish a connection to a line-based server at the provided `addr`.
    pub fn connect(addr: &SocketAddr, handle: &Handle) -> Box<Future<Item = TcpClient, Error = io::Error>> {
        let ret = tokio_proto::TcpClient::new(client::RiemannProto)
            .connect(addr, handle)
            .map(|client_service| {
                TcpClient { inner: client_service }
            });
        Box::new(ret)
    }
}

impl Service for TcpClient {
    type Request = client::MessageFrame;
    type Response = client::MessageFrame;
    type Error = io::Error;
    // For simplicity, box the future.
    type Future = Box<Future<Item = client::MessageFrame, Error = io::Error>>;

    fn call(&self, req: client::MessageFrame) -> Self::Future {
        let b = Box::new(self.inner.call(req).and_then(|resp| {
            Ok(resp)
        }));        b
    }
}
