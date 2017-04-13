#[macro_use]
extern crate clap;
extern crate riemann_rust;
extern crate tokio_service;
extern crate tokio_core;
pub mod cli;
pub mod util;
use clap::App;
use tokio_service::Service;
use tokio_core::reactor::Core;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(matches) = matches.subcommand_matches("send") {
        let protocol = matches.value_of("protocol").unwrap();
        let riemann_server = matches.value_of("server").unwrap_or("127.0.0.1");
        let port = matches.value_of("port").unwrap_or("5555");

        let event = cli::get_event(&matches).unwrap();
        let addr = format!("{}:{}", riemann_server, port).parse().unwrap();
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let _ = match protocol {
            "tcp" => {
                let c = core.run(riemann_rust::tcp::TcpClient::connect(&addr, &handle));
                match c {
                    Ok(client) => {
                        let result = core.run(client.call(riemann_rust::codec::get_frame(event)));
                        match result {
                            Ok(r) => println!("result : {:?}", r),
                            Err(error) => {
                                println!("error during send : {}", error)
                            }
                        }
                    },
                    Err(err) => {
                        println!("Error during send : {}", err);
                        std::process::exit(2);
                    }
                }
            },
            _ => {
                println!("Unknown protocol : {}", protocol);
                std::process::exit(1);
            }
        };
        
    }
}
