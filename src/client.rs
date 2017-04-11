use event;
use proto::proto;
use std::io;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use protobuf::Message;
use byteorder::BigEndian;
use protobuf;
use bytes::{BytesMut};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_core::net::{UdpCodec};
use std::net::SocketAddr;
use tokio_io::codec::{Encoder, Decoder, Framed};
use tokio_proto::pipeline::{ClientProto};

const LENGTH_LEN: usize = 4;

pub struct MessageCodec;

#[derive(Debug)]
pub struct MessageFrame {
    pub message: proto::Msg,
    pub length: u32
}

/******** UDP *********/
impl UdpCodec for MessageCodec {
    type In = (SocketAddr, MessageFrame);
    type Out = (SocketAddr, MessageFrame);

    fn encode(&mut self, (addr, msg): Self::Out, into: &mut Vec<u8>) -> SocketAddr {
        let mut content_writer = vec![];
        let _ = msg.message.write_to_vec(&mut content_writer);
        into.extend(content_writer);
        addr
    }

    fn decode(&mut self, addr: &SocketAddr, buf: &[u8]) -> io::Result<Self::In> {
        let msg = (&buf[0..4]).read_u32::<BigEndian>();
        match msg {
            Ok(cl) => {
                let content_length = cl as usize;
                let total_len = LENGTH_LEN + content_length;
                let message = protobuf::parse_from_bytes::<proto::Msg>(&buf[LENGTH_LEN..total_len]);
                match message {
                    Ok(msg) => (Ok((*addr, MessageFrame {
                        message: msg,
                        length: cl}))),
                    Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                                 "Error proto msg content"))
                }
            },
            Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                         "Error proto msg decoding"))
        }
    }
}

/******** Codec *********/
impl Encoder for MessageCodec {
    type Item = MessageFrame;
    type Error = io::Error;

    fn encode(&mut self, msg: MessageFrame, buf: &mut BytesMut) -> io::Result<()> {
        // contains length
        let mut len_writer = vec![];
        len_writer.write_u32::<BigEndian>(msg.length)?;
        buf.extend(len_writer);
        // contains content
        let mut content_writer = vec![];
        let _ = msg.message.write_to_vec(&mut content_writer);
        buf.extend(content_writer);
        Ok(())
    }
}

impl Decoder for MessageCodec {
    type Item = MessageFrame;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<MessageFrame>> {
        if buf.len() < LENGTH_LEN {
            // wait for length
            Ok(None)
        }
        else {
            let msg = (&buf[0..4]).read_u32::<BigEndian>();
            match msg {
                Ok(cl) => {
                    let content_length = cl as usize;
                    let total_len = LENGTH_LEN + content_length;
                    if buf.len() < total_len {
                        // wait for rest of msg
                        Ok(None)
                    }
                    else {
                        let new_buf = buf.split_to(total_len);
                        let message = protobuf::parse_from_bytes::<proto::Msg>(&new_buf[LENGTH_LEN..total_len]);
                        match message {
                            Ok(msg) => Ok(Some(MessageFrame {
                                message: msg,
                                length: cl})),
                            Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                             "Error proto msg content"))
                        }
                    }
                },
                Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                             "Error proto msg size"))
            }
        }
    }
}

/******** Proto *********/
pub struct RiemannProto;

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for RiemannProto {
    type Request = MessageFrame;
    type Response = MessageFrame;

    type Transport = Framed<T, MessageCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(MessageCodec))
    }
}
