use std::{
    io::{StdoutLock, Write, stdout},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    result::{
        Result,
        Result::{Err, Ok},
    },
};

pub fn web_service() -> () {
    let socket_address: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    let transmission_listener: TcpListener = TcpListener::bind(socket_address).unwrap();
    let transmission_stream: TcpStream = TcpStream::connect(socket_address).unwrap();

    println!("Service Listening on Port: 8080");

    return ();
}
