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
pub mod udp;

use std::net::SocketAddr;
use byteorder::WriteBytesExt;
use byteorder::BigEndian;
use bytes::{BytesMut, BufMut};
use futures::{Stream, Sink};
use tokio_core::net::{UdpSocket, UdpCodec};
use futures::Future;
use tokio_core::reactor::Core;
use tokio_service::Service;
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
    let frame = client::MessageFrame {
        message: msg,
        length: size
    };

    let mut e2 = proto::proto::Event::new();
    e2.set_state("critical".to_owned());
    e2.set_service("rust222".to_owned());
    e2.set_host("bar".to_owned());

    let mut msg2 = proto::proto::Msg::new();
    msg2.set_events(RepeatedField::from_vec(vec![e2]));

    let size2 = msg2.compute_size();
    let frame2 = client::MessageFrame {
        message: msg2,
        length: size2
    };

    let handle = core.handle();
//    let addr = "127.0.0.1:5555".parse().unwrap();
    
    // let client = core.run(tcp::TcpClient::connect(&addr, &handle));
    
    // match client {
    //     Ok(c) => {
    //         let response = core.run(c.call(frame));
    //         match response {
    //             Ok(resp) => {
    //                 println!("=> {:?}", resp);
    //                 ()
    //             },
    //             Err(e) => ()
    //         }
    //     }
    //     Err(e) => ()
    // }
    let addr2: SocketAddr = "127.0.0.1:0".parse().unwrap();

    let a = UdpSocket::bind(&addr2, &handle).unwrap();

    let addr3: SocketAddr = "127.0.0.1:5555".parse().unwrap();
    let (a_sink, a_stream) = a.framed(client::MessageCodec).split();
    let a = a_sink.send((addr3, frame));
    core.run(a);
    // let _ = core.run(
    //     let fclient = tcp::TcpClient::connect(&addr, &handle);
    //     Ok(())
    //         // .and_then(|client| {
                
    //         //     for x in 0..10 {
    //         //         println!("{}", x); // x: i32
    //         //         client.call(frame);
    //         //         ()
    //         //     }
    //         // }
    //             // client.call(frame)
    //             //     .and_then(move |response| {
    //             //         println!("CLIENT: {:?}", response);
    //             //         client.call(frame2)
    //             //     })
    //             //     .and_then(|response| {
    //             //         println!("CLIENT: {:?}", response);
    //             //         Ok(())
    //             //     })
            
    //         )
}
