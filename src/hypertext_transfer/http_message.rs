use std::{
    io::{BufReader, Read, StdoutLock, Write, stdout},
    net::TcpStream,
    string::String,
};

// Hypertext Transfer Protocol Body Definition
pub type HttpBody<T> = T;

// Hypertext Transfer Protocol Connection Management
pub fn manage_connection(transmission_stream: &TcpStream) -> () {
    let mut standard_output: StdoutLock = stdout().lock();
    let mut buffered_reader: BufReader<&TcpStream> = BufReader::new(&transmission_stream);
    let mut stream_buffer: String = String::new();

    buffered_reader.read_to_string(&mut stream_buffer).unwrap();
    writeln!(standard_output, "Hypertext Tranfer Protocol Request: ").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "{}", stream_buffer).unwrap();

    return ();
}
