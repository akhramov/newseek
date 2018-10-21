use std::str::FromStr;
use std::time::Duration;
use std::{io, net, process, thread};

use actix::prelude::*;
use futures::Future;
use tokio_codec::FramedRead;
use tokio_io::io::WriteHalf;
use tokio_io::AsyncRead;
use tokio_tcp::TcpStream;
use futures;

use errors;
use domain::*;

mod codec;

pub fn listen(handler: &'static (Fn(TcpStream) -> ()))  {
    let addr = net::SocketAddr::from_str("0.0.0.0:3011").unwrap();
    Arbiter::spawn(
        TcpStream::connect(&addr)
            .and_then(move |stream| {
                handler(stream);

                futures::future::ok(())
            })
            .map_err(|e| {
                println!("Can not connect to server: {}", e);
                process::exit(1)
            }));
}

pub fn create_client(stream: TcpStream) -> Addr<impl Actor> {
    InterfaceClient::create(|ctx| {
        let (r, w) = stream.split();
        ctx.add_stream(FramedRead::new(r, codec::Codec));
        InterfaceClient {
            framed: actix::io::FramedWrite::new(
                w,
                codec::Codec,
                ctx,
            ),
        }
    })
}

#[derive(Message)]
pub enum InterfaceClientMessage {
    PrivateMessage(PrivateMessageResponse)
}

pub struct InterfaceClient {
    framed: actix::io::FramedWrite<WriteHalf<TcpStream>, codec::Codec>,
}

impl actix::io::WriteHandler<errors::Error> for InterfaceClient {}

impl StreamHandler<codec::Request, errors::Error> for InterfaceClient {
    fn handle(&mut self, msg: codec::Request, _: &mut Context<Self>) {
        match msg {
            codec::Request::Login(ref msg) => {
                // println!("message: {:?}", msg);
            },
            codec::Request::Challenge(ref msg) => {
                let response =
                    LoginResponse::new("password".to_owned(), msg.challenge.clone());

                self.framed.write(codec::Response::Login(response));
            },
            codec::Request::Transfers(ref msg) => {
                println!("message: {:?}", msg);

                // self.framed.write(codec::Response::PrivateMessage(PrivateMessageResponse::new("foobjewje".to_string(), "Hey there".to_string())));
            },
            _ => (),
        }
    }
}

impl Handler<InterfaceClientMessage> for InterfaceClient {
    type Result = ();

    fn handle(&mut self, msg: InterfaceClientMessage, _: &mut Context<Self>) {
        match msg {
            InterfaceClientMessage::PrivateMessage(message) => {
                self.framed.write(codec::Response::PrivateMessage(message));
            }
        }
    }
}


impl Actor for InterfaceClient {
    type Context = Context<Self>;
}
