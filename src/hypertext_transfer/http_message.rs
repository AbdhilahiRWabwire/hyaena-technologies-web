use std::{
    fs,
    io::{BufReader, Read, StdoutLock, Write, stdout},
    net::{TcpListener, TcpStream},
    option::Option,
    path::PathBuf,
    primitive::usize,
    string::String,
    vec::Vec,
};

use super::{
    http_headers::HTTPHeader,
    http_methods::HTTPMethod,
    http_security_directives::HTTPSecurityDirective,
    http_status_codes::{HTTPStatusCode, HTTPStatusText},
    http_versions::HTTPVersion,
};

use crate::hypertext_transfer::{
    http_headers::HTTP_CONTENT_LENGTH,
    http_status_codes::{HTTP_OK, HTTP_TWO_HUNDRED},
    http_versions::HTTP_VERSION_ONE,
};

// Hypertext Transfer Protocol Body Definition
pub type HTTPBody<T> = T;

// Hypertext Transfer Protocol Message Definition
pub struct HypertextTransferMessage<T> {
    pub body: Option<HTTPBody<T>>,
    pub headers: Vec<HTTPHeader>,
    pub method: HTTPMethod,
    pub security_directives: Option<Vec<HTTPSecurityDirective>>,
    pub status_code: HTTPStatusCode,
    pub status_text: HTTPStatusText,
    pub version: HTTPVersion,
}

// Hypertext Transfer Protocol Connection Management
pub fn http_connection_management(transmission_stream: &mut TcpStream) -> () {
    let mut standard_output: StdoutLock = stdout().lock();
    let mut buffered_reader: BufReader<&TcpStream> = BufReader::new(&transmission_stream);
    let mut stream_buffer: String = String::new();
    let source_path: PathBuf = PathBuf::from("./web/source/main.js");
    let source_file: String = fs::read_to_string(source_path).unwrap();
    let content_length: usize = source_file.len();

    buffered_reader.read_to_string(&mut stream_buffer).unwrap();
    writeln!(standard_output, "Hypertext Tranfer Protocol Request: ").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "{}", stream_buffer).unwrap();

    writeln!(
        transmission_stream,
        "{} {} {}",
        HTTP_VERSION_ONE.to_string(),
        HTTP_TWO_HUNDRED.to_string(),
        HTTP_OK.to_string()
    )
    .unwrap();
    writeln!(
        transmission_stream,
        "{}: {}",
        HTTP_CONTENT_LENGTH.to_string(),
        content_length
    )
    .unwrap();
    writeln!(transmission_stream, "{}", source_file).unwrap();

    return ();
}

// Hypertext Transer Protocol Client
pub fn http_client() -> () {
    return ();
}

// Hypertext Transfer Protocol Server
pub fn http_server(transmission_listener: TcpListener) -> () {
    transmission_listener.set_ttl(100).unwrap();

    for transmission_stream in transmission_listener.incoming() {
        let stream: TcpStream = transmission_stream.unwrap();

        stream.set_ttl(100).unwrap();
        http_connection_management(&stream);
    }

    return ();
}
