#[macro_use]
extern crate clap;
extern crate rustmann;
pub mod cli;
use rustmann::tcp;
use rustmann::udp;
use rustmann::event;
use std::time::Duration;
use rustmann::client::Client;
use clap::App;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;

fn main() {

    // let mut client = tcp::TcpClient::new("127.0.0.1", 5555);
    // let r = client.connect(Duration::new(5, 0));
    // let mut e = event::Event::new();
    // e.host = Some("localhost".to_owned());
    // let result = client.send(&vec![e]);
    // println!("result : {:?}", result);


    // let mut client = udp::UdpClient::new("127.0.0.1", 5555, "127.0.0.1", None);
    // let r = client.connect(Duration::new(5, 0));
    // println!("connected : {:?}", r);
    // let mut e = event::Event::new();
    // e.host = Some("localhost".to_owned());
    // let result = client.send(&vec![e]);
    // println!("result : {:?}", result);
//    let yaml = load_yaml!("cli.yaml");
//    let matches = App::from_yaml(yaml).get_matches();
    // if let Some(matches) = matches.subcommand_matches("send") {
    //     let protocol = matches.value_of("protocol").unwrap();
    //     let riemann_server = matches.value_of("server").unwrap_or("127.0.0.1");
    //     let port = matches.value_of("port").unwrap_or("5555");
    //     let addr: SocketAddr = format!("{}:{}", riemann_server, port).parse().unwrap();
    //     let event = cli::get_event(&matches).unwrap();
    // }
}
