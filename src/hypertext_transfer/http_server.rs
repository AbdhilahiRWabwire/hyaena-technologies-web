#![allow(dead_code)]
#![allow(unused_variables)]

use std::net::{TcpListener, TcpStream};

// Hypertext Transer Protocol Server Definition
pub struct HttpServer {}

// Hypertext Transfer Protocol Server
pub fn protocol_server(server: HttpServer, transmission_listener: TcpListener) -> HttpServer {
    for transmission_stream in transmission_listener.incoming() {
        let stream: TcpStream = transmission_stream.unwrap();
    }

    return server;
}
