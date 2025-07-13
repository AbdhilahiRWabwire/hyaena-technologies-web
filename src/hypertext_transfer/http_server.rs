#![allow(dead_code)]

use std::net::{TcpListener, TcpStream};

use crate::hypertext_transfer::http_message::manage_connection;

// Hypertext Transer Protocol Server Definition
pub struct HttpServer {}

// Hypertext Transfer Protocol Server
pub fn protocol_server(server: HttpServer, transmission_listener: TcpListener) -> HttpServer {
    for transmission_stream in transmission_listener.incoming() {
        let mut stream: TcpStream = transmission_stream.unwrap();

        manage_connection(&mut stream);
    }

    return server;
}
