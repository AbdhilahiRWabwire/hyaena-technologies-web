#![allow(dead_code)]
#![allow(unused_variables)]

use std::net::{TcpListener, TcpStream};

use crate::hypertext_transfer::http_message::manage_connection;

// Hypertext Transer Protocol Server Definition
pub struct HttpServer {}

// Hypertext Transfer Protocol Server
pub fn protocol_server(server: HttpServer, transmission_listener: TcpListener) -> () {
    transmission_listener.set_ttl(100).unwrap();

    for transmission_stream in transmission_listener.incoming() {
        let mut stream: TcpStream = transmission_stream.unwrap();

        stream.set_ttl(100).unwrap();
        manage_connection(&mut stream);
    }

    return ();
}
