use std::{
    io::{Error, StdoutLock, Write, stdout},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    process::exit,
    result::{
        Result,
        Result::{Err, Ok},
    },
    thread,
    thread::JoinHandle,
};

use crate::{hypertext_transfer::http_message::manage_connection, routing::home_page::home_route};

// Application Service
pub fn web_service() -> () {
    let socket_address: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 7878);
    let transmission_listener: Result<TcpListener, Error> = TcpListener::bind(socket_address);
    let mut standard_output: StdoutLock = stdout().lock();

    match transmission_listener {
        Ok(listener) => {
            writeln!(standard_output, "Service Listening on Port: 7878").unwrap();

            for transmission_stream in listener.incoming() {
                let mut stream: TcpStream = transmission_stream.unwrap();

                manage_connection(&mut stream);
                home_route(&mut stream);
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
