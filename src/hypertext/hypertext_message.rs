#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{
    io::{BufRead, BufReader, Error, StdoutLock, Write, stdout},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    result::{
        Result,
        Result::{Err, Ok},
    },
    string::String,
    vec::Vec,
};

// Hypertext Transfer Protocol Message Definition
pub struct HypertextMessage<T> {
    pub body: T,
    pub headers: Vec<String>,
    pub method: String,
    pub status_code: String,
    pub status_text: String,
    pub version: String,
}

// Hypertext Transfer Protocol Connection Management
pub fn hypertext_connection_management(mut transmission_stream: TcpStream) {
    let mut standard_output: StdoutLock = stdout().lock();
    let buffered_reader: BufReader<&TcpStream> = BufReader::new(&transmission_stream);
    let http_request: Vec<String> = buffered_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    writeln!(
        standard_output,
        "Hypertext Tranfer Protocol Request: {:#?}",
        http_request
    )
    .unwrap();
    writeln!(transmission_stream, "HTTP/1.1 200 OK").unwrap();
}

// Hypertext Transer Protocol Client
pub fn hypertext_client() -> () {
    return ();
}

// Hypertext Transfer Protocol Server
pub fn hypertext_server() -> () {
    return ();
}
