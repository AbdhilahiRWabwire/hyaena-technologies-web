use std::{
    io::{Error, StdoutLock, Write, stdout},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    process::exit,
    result::{
        Result,
        Result::{Err, Ok},
    },
};

use crate::hypertext_transfer::http_message::http_connection_management;

// Application Service
pub fn web_service() -> () {
    let socket_address: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 7878);
    let transmission_listener: Result<TcpListener, Error> = TcpListener::bind(socket_address);
    let mut standard_output: StdoutLock = stdout().lock();

    match transmission_listener {
        Ok(listener) => {
            listener.set_ttl(100).unwrap();
            writeln!(standard_output, "Service Listening on Port: 7878").unwrap();

            for transmission_stream in listener.incoming() {
                let mut stream: TcpStream = transmission_stream.unwrap();

                stream.set_ttl(100).unwrap();
                http_connection_management(&mut stream);
            }
        }
        Err(error) => {
            eprintln!(
                "Error Could Not Initialize Transmission Listener: {}",
                error
            );
            exit(1);
        }
    };

    return ();
}
