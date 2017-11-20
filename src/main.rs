#[macro_use]
extern crate clap;
extern crate rustmann;
extern crate chrono;
pub mod cli;
use rustmann::tcp;
use rustmann::udp;
use std::time::Duration;
use rustmann::client::Client;
use rustmann::client::Index;
use clap::App;
use std::net::SocketAddr;

fn main() {

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(matches) = matches.subcommand_matches("send") {
        let protocol = matches.value_of("protocol").unwrap();
        let host = matches.value_of("server").unwrap_or("127.0.0.1");
        let port = matches.value_of("port").unwrap_or("5555");
        // todo error handling
        let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();;
        let event = cli::get_event(&matches).unwrap();
        if protocol == "udp" {
            let bind_addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
            let mut client = udp::UdpClient::new(addr, bind_addr);
            let connect_result = client.connect(Duration::from_secs(5));
            let result = client.send(&vec![event]);
            println!("event send {:?}", result);
        }
        else if protocol == "tcp" {
            let mut client = tcp::TcpClient::new(addr);
            let connect_result = client.connect(Duration::from_secs(5));
            let result = client.send(&vec![event]);
            println!("event send {:?}", result);
        }
    }
    else if let Some(matches) = matches.subcommand_matches("query") {
        let protocol = matches.value_of("protocol").unwrap();
        let host = matches.value_of("server").unwrap_or("127.0.0.1");
        let port = matches.value_of("port").unwrap_or("5555");
        let query = matches.value_of("query").unwrap_or("true");
        // todo error handling
        let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();;
        let mut client = tcp::TcpClient::new(addr);
        let connect_result = client.connect(Duration::from_secs(5));
        let result = client.query(query.to_owned());
        println!("event send {:?}", result);

    }
}
