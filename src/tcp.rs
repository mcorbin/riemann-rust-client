use event;
use proto::proto;
extern crate tokio_proto;
extern crate tokio_service;
use std::io;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use protobuf::Message;
use byteorder::BigEndian;
use protobuf;
use bytes::{BytesMut};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Encoder, Decoder, Framed};
use tokio_proto::pipeline::{ServerProto, ClientProto, ClientService};
use tokio_service::{Service, NewService};

const LENGTH_LEN: usize = 4;

pub struct MessageCodec;

pub struct MessageFrame {
    pub message: proto::Msg,
    pub length: u32
}

impl Encoder for MessageCodec {
    type Item = MessageFrame;
    type Error = io::Error;

    fn encode(&mut self, msg: MessageFrame, buf: &mut BytesMut) -> io::Result<()> {
        let mut len_writer = vec![];
        len_writer.write_u32::<BigEndian>(msg.length)?;
        buf.extend(len_writer);
        let mut content_writer = vec![];
        msg.message.write_to_vec(&mut content_writer);
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
            let msg = (&buf[0..3]).read_u32::<BigEndian>();
            match msg {
                Ok(cl) => {
                    let content_length = cl as usize;
                    let total_len = LENGTH_LEN + content_length;
                    if buf.len() < total_len {
                        // wait for rest of msg
                        Ok(None)
                    }
                    else {
                        let message = protobuf::parse_from_bytes::<proto::Msg>(&buf[LENGTH_LEN..total_len]);
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
                                             "Error proto msg size")),
            }
        }
    }
}

// proto

pub struct RiemannProto;

impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for RiemannProto {
    type Request = MessageFrame;
    type Response = MessageFrame;

    /// `Framed<T, LineCodec>` is the return value of `io.framed(LineCodec)`
    type Transport = Framed<T, MessageCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(MessageCodec))
    }
}
