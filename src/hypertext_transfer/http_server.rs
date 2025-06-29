#![allow(dead_code)]

use std::net::{TcpListener, TcpStream};

use crate::hypertext_transfer::http_message::http_connection_management;

// Hypertext Transer Protocol Server
pub struct HTTPServer {}

// Hypertext Transfer Protocol Server
pub fn server(transmission_listener: TcpListener) -> () {
    transmission_listener.set_ttl(100).unwrap();

    for transmission_stream in transmission_listener.incoming() {
        let mut stream: TcpStream = transmission_stream.unwrap();

        stream.set_ttl(100).unwrap();
        http_connection_management(&mut stream);
    }

    return ();
}
