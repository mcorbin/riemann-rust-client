#[macro_use]
extern crate clap;
extern crate rustmann;
pub mod cli;
use clap::App;
use std::net::SocketAddr;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    // if let Some(matches) = matches.subcommand_matches("send") {
    //     let protocol = matches.value_of("protocol").unwrap();
    //     let riemann_server = matches.value_of("server").unwrap_or("127.0.0.1");
    //     let port = matches.value_of("port").unwrap_or("5555");
    //     let addr: SocketAddr = format!("{}:{}", riemann_server, port).parse().unwrap();
    //     let event = cli::get_event(&matches).unwrap();
    // }
}
