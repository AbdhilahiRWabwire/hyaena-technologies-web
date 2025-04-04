#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    string::String,
};

pub fn web_service() -> () {
    let socket_address: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    let transmission_listener: TcpListener = TcpListener::bind(socket_address).unwrap();
    let transmission_stream: TcpStream = TcpStream::connect(socket_address).unwrap();

    return ();
}
