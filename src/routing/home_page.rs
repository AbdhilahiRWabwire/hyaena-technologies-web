use std::{
    fs::File,
    io::{Error, Read, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    path::PathBuf,
    primitive::usize,
    process::exit,
    result::Result::{self, Err, Ok},
    string::String,
};

use crate::hypertext_transfer::{
    http_headers::HTTP_CONTENT_LENGTH,
    http_status_codes::{HTTP_OK, HTTP_TWO_HUNDRED},
    http_versions::HTTP_VERSION_ONE,
};

// Application Home Page Route
pub fn home_route() -> () {
    let socket_address: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 7878);
    let transmission_listener: Result<TcpListener, Error> = TcpListener::bind(socket_address);

    match transmission_listener {
        Ok(listener) => {
            for transmission_stream in listener.incoming() {
                let source_path: PathBuf = PathBuf::from("./web/src/main.js");
                let source_file: Result<File, Error> = File::open(source_path);
                let mut file_buffer: String = String::new();
                let content_length: usize = file_buffer.len();
                let mut stream: TcpStream = transmission_stream.unwrap();

                match source_file {
                    Ok(mut file) => {
                        file.read_to_string(&mut file_buffer).unwrap();
                        writeln!(
                            stream,
                            "{} {} {}",
                            HTTP_VERSION_ONE.to_string(),
                            HTTP_TWO_HUNDRED.to_string(),
                            HTTP_OK.to_string()
                        )
                        .unwrap();
                        writeln!(
                            stream,
                            "{}: {}",
                            HTTP_CONTENT_LENGTH.to_string(),
                            content_length
                        )
                        .unwrap();
                        writeln!(stream, "{}", file_buffer).unwrap();
                    }
                    Err(error) => {
                        eprintln!("Error Opening File: {}", error);
                        exit(1);
                    }
                }
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
