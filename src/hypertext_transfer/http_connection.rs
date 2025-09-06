use std::{
    io::{BufReader, Error, Read, StdoutLock, Write, stdout},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream, UdpSocket},
    process::exit,
    result::{
        Result,
        Result::{Err, Ok},
    },
    string::String,
    thread,
};

pub struct HttpConnection {
    pub ip_address: IpAddr,
    pub socket_address: SocketAddr,
    pub transmission_listener: Result<TcpListener, Error>,
    pub datagram_socket: Result<UdpSocket, Error>,
}

// Print Hypertext Transfer Protocol Requests
pub fn print_requests(connection: HttpConnection) -> () {
    let tcp_listener: Result<TcpListener, Error> = connection.transmission_listener;

    match tcp_listener {
        Ok(listener) => {
            for transmission_stream in listener.incoming() {
                thread::spawn(|| {
                    let mut standard_output: StdoutLock = stdout().lock();
                    let stream: TcpStream = transmission_stream.unwrap();
                    let mut buffered_reader: BufReader<&TcpStream> = BufReader::new(&stream);
                    let mut stream_buffer: String = String::new();

                    buffered_reader.read_to_string(&mut stream_buffer).unwrap();
                    writeln!(standard_output, "Hypertext Tranfer Protocol Request: ").unwrap();
                    writeln!(standard_output, "").unwrap();
                    writeln!(standard_output, "{}", stream_buffer).unwrap();
                });
            }
        }
        Err(error) => {
            eprintln!("Error Initializing Transmission Listener: {}", error);
            exit(1);
        }
    };

    return ();
}
