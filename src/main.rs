extern crate byteorder;
extern crate bytes;
extern crate protobuf;
extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_io;
pub mod proto;
pub mod client;
pub mod codec;
pub mod event;
pub mod tcp;

use futures::Future;
use tokio_core::reactor::Core;
use tokio_service::Service;
use std::thread;
use std::time::Duration;
use protobuf::{RepeatedField, Message};

fn main() {
    let mut core = Core::new().unwrap();

    let mut e = proto::proto::Event::new();
    e.set_state("critical".to_owned());
    e.set_service("rust".to_owned());
    e.set_host("bar".to_owned());

    let mut msg = proto::proto::Msg::new();
    msg.set_events(RepeatedField::from_vec(vec![e]));

    let size = msg.compute_size();
    let frame = tcp::MessageFrame {
        message: msg,
        length: size
    };

    // let mut e2 = proto::proto::Event::new();
    // e2.set_state("critical".to_owned());
    // e2.set_service("rust222".to_owned());
    // e2.set_host("bar".to_owned());

    // let mut msg2 = proto::proto::Msg::new();
    // msg2.set_events(RepeatedField::from_vec(vec![e2]));

    // let size2 = msg2.compute_size();
    // let frame2 = tcp::MessageFrame {
    //     message: msg2,
    //     length: size2
    // };

    let handle = core.handle();
    let addr = "127.0.0.1:5555".parse().unwrap();
    let result = core.run(
        tcp::Client::connect(&addr, &handle)
            .and_then(|client| {
                client.call(frame)
                    .and_then(move |response| {
                        println!("CLIENT: {:?}", response);
                        //                        client.call(frame2)
                        Ok(())
                    })
                    // .and_then(|response| {
                    //     println!("CLIENT: {:?}", response);
                    //     Ok(())
                    // })
            })
    );
    println!("RESULT: {:?}", result);
}
