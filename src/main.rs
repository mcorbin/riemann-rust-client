#[macro_use]
extern crate clap;
extern crate riemann_rust;
extern crate futures;
extern crate tokio_service;
extern crate tokio_core;
extern crate rustls;
extern crate tokio_rustls;

pub mod cli;
pub mod util;
pub mod tls;
use std::net::SocketAddr;
use clap::App;
use tokio_service::Service;
use tokio_core::reactor::Core;
use tokio_core::net::UdpSocket;
use futures::{Stream, Sink, Future};
use tokio_rustls::ClientConfigExt;
use tokio_core::net::TcpStream;

fn send_tls(event: riemann_rust::event::Event, addr: &SocketAddr) {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let c = core.run(riemann_rust::tcp::TcpClient::connect(&addr, &handle));
    match c {
        Ok(client) => {
            let config = tls::get_config("/home/mathieu/prog/go/ssl/client.key",
                                         "/home/mathieu/prog/go/ssl/client.pem",
                                         "/home/mathieu/prog/go/ssl/ca.pem");
            c.and_then(|socket| config.connect_async("localhost", socket));
            
	        

        },
        Err(err) => {
            println!("Error during send : {}", err);
            std::process::exit(2);
        }
    }
}

fn send_tcp(event: riemann_rust::event::Event, addr: &SocketAddr) {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let c = core.run(riemann_rust::tcp::TcpClient::connect(&addr, &handle));
    match c {
        Ok(client) => {
            let result = core.run(client.call(riemann_rust::codec::get_frame(event)));
            match result {
                Ok(r) => println!("result : {:?}", r),
                Err(error) => {
                    println!("Error during send : {}", error)
                }
            }
        },
        Err(err) => {
            println!("Error during send : {}", err);
            std::process::exit(2);
        }
    }
}

fn send_udp(event: riemann_rust::event::Event, riemann_addr: &SocketAddr) {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let s1 = UdpSocket::bind(&addr, &handle).unwrap();
    let (a_sink, _) = s1.framed(riemann_rust::client::MessageCodec).split();
    let frame = riemann_rust::codec::get_frame(event);
    let _ = core.run(a_sink.send((*riemann_addr, frame)));
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(matches) = matches.subcommand_matches("send") {
        let protocol = matches.value_of("protocol").unwrap();
        let riemann_server = matches.value_of("server").unwrap_or("127.0.0.1");
        let port = matches.value_of("port").unwrap_or("5555");
        let addr: SocketAddr = format!("{}:{}", riemann_server, port).parse().unwrap();
        let event = cli::get_event(&matches).unwrap();
        let _ = match protocol {
            "tcp" => {
                send_tcp(event, &addr);
            },
            "udp" => {
                send_udp(event, &addr);
            }
            _ => {
                println!("Unknown protocol : {}", protocol);
                std::process::exit(1);
            }
        };
    }
}
