use std::{
    io::{BufReader, Read, StdoutLock, Write, stdout},
    net::{IpAddr, SocketAddr, TcpStream, UdpSocket},
    option::Option,
    process::exit,
    string::String,
    thread,
    thread::JoinHandle,
};

// Hypertext Transfer Protocol Body Definition
pub type HttpBody<T> = T;

pub struct HttpConnection {
    pub ip_address: IpAddr,
    pub socket_address: SocketAddr,
    pub transmission_stream: Option<TcpStream>,
    pub datagrm_socket: Option<UdpSocket>,
}

// Print Hypertext Transfer Protocol Requests
pub fn print_requests(connection: HttpConnection) -> () {
    let standard_thread: JoinHandle<()> = thread::spawn(|| {
        let stream: Option<TcpStream> = connection.transmission_stream;

        match stream {
            Some(stream) => {
                let mut standard_output: StdoutLock = stdout().lock();
                let mut buffered_reader: BufReader<&TcpStream> = BufReader::new(&stream);
                let mut stream_buffer: String = String::new();

                buffered_reader.read_to_string(&mut stream_buffer).unwrap();
                writeln!(standard_output, "Hypertext Tranfer Protocol Request: ").unwrap();
                writeln!(standard_output, "").unwrap();
                writeln!(standard_output, "{}", stream_buffer).unwrap();
            }
            None => {
                eprintln!("Error Connecting To Transmission Listener");
                exit(1);
            }
        }
    });

    standard_thread.join().unwrap();

    return ();
}
