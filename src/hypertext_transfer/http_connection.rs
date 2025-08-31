use std::{
    io::{BufReader, Read, StdoutLock, Write, stdout},
    net::{IpAddr, SocketAddr, TcpStream, UdpSocket},
    string::String,
};

pub struct HttpConnection {
    pub ip_address: IpAddr,
    pub socket_address: SocketAddr,
    pub transmission_stream: TcpStream,
    pub datagrm_socket: UdpSocket,
}

// Print Hypertext Transfer Protocol Requests
pub fn print_requests(connection: HttpConnection) -> () {
    let mut standard_output: StdoutLock = stdout().lock();
    let stream: TcpStream = connection.transmission_stream;
    let mut buffered_reader: BufReader<&TcpStream> = BufReader::new(&stream);
    let mut stream_buffer: String = String::new();

    buffered_reader.read_to_string(&mut stream_buffer).unwrap();
    writeln!(standard_output, "Hypertext Tranfer Protocol Request: ").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "{}", stream_buffer).unwrap();

    return ();
}
