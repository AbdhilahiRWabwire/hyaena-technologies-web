use std::{
    io::{StdoutLock, Write, stdout},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    result::{
        Result,
        Result::{Err, Ok},
    },
};

use super::super::hypertext::hypertext_message::hypertext_connection;

pub fn web_service() -> () {
    let socket_address: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    let transmission_listener: TcpListener = TcpListener::bind(socket_address).unwrap();
    let mut standard_output: StdoutLock = stdout().lock();

    transmission_listener.set_ttl(100).unwrap();
    writeln!(standard_output, "Service Listening on Port: 8080").unwrap();

    for transmission_stream in transmission_listener.incoming() {
        let stream: TcpStream = transmission_stream.unwrap();

        stream.set_ttl(100).unwrap();
        hypertext_connection(stream);
    }

    return ();
}
