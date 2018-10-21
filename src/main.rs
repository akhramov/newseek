#[macro_use]
extern crate serde_derive;
extern crate actix;
extern crate newseek;
extern crate tokio_tcp;

use actix::prelude::*;

use std::fs;

use newseek::interface;
use newseek::domain::*;
use newseek::binary_format::de;

use newseek::repository::storage::{
    Store,
    PgStore,
};

use newseek::web;
use tokio_tcp::TcpStream;


#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct SearchResult {
    path: String,
    size: u64,
    file_type: String,
    attributes: Vec<u32>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct SearchReply {
    ticket: u32,
    user: String,
    free: bool,
    speed: u32,
    queue: u32,
    results: Vec<SearchResult>
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");

    let sys = actix::System::new("ws-example");

    interface::listen(&handler);

    let _ = sys.run();

    // let client = interface::InterfaceClient.get();

    // client.send(
    //     interface::InterfaceClientMessage::PrivateMessage(
    //         PrivateMessageResponse::new(
    //             "foobjewje".to_owned(),"Hey there".to_owned()
    //         )));


    // let store = PgStore::new();

    // let mut encoded = fs::read("/Users/artem/Downloads/search").unwrap();

    // let decoded: SearchReply = de::from_slice(&mut encoded[..]).unwrap();

    // let mut encoded2 = ser::to_slice(&decoded).unwrap();

    // let decoded2: SearchReply = de::from_slice(&mut encoded2[..]).unwrap();

    // assert_eq!(encoded, encoded2);
}

fn handler(stream: TcpStream) {
    interface::create_client(stream);

    web::server();
    println!("Started http server: 127.0.0.1:8081");
}
