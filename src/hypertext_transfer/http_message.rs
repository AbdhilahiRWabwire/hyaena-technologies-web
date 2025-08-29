use std::{
    fs::File,
    io::{BufReader, Error, Read, StdoutLock, Write, stdout},
    net::TcpStream,
    option::Option,
    path::PathBuf,
    primitive::usize,
    process::exit,
    result::{
        Result,
        Result::{Err, Ok},
    },
    string::String,
    vec::Vec,
};

use super::{
    http_headers::HttpHeader,
    http_methods::HttpMethod,
    http_security_directives::HttpSecurityDirective,
    http_status_codes::{HttpStatusCode, HttpStatusText},
    http_versions::HttpVersion,
};

// Hypertext Transfer Protocol Body Definition
pub type HttpBody<T> = T;

// Hypertext Transfer Protocol Request Definition
pub struct HttpRequest<T> {
    pub body: Option<HttpBody<T>>,
    pub headers: Vec<HttpHeader>,
    pub method: HttpMethod,
    pub path: String,
    pub security_directives: Option<Vec<HttpSecurityDirective>>,
    pub status_code: HttpStatusCode,
    pub status_text: HttpStatusText,
    pub version: HttpVersion,
}

// Hypertext Transfer Protocol Response Definition
pub struct HttpResponse<T> {
    pub body: Option<HttpBody<T>>,
    pub headers: Vec<HttpHeader>,
    pub method: HttpMethod,
    pub security_directives: Option<Vec<HttpSecurityDirective>>,
    pub status_code: HttpStatusCode,
    pub status_text: HttpStatusText,
    pub version: HttpVersion,
}

// Hypertext Transfer Protocol Request Message
pub fn request_message<T>(request: HttpRequest<T>) -> HttpRequest<T> {
    return request;
}

// Hypertext Transfer Protocol Response Message
pub fn response_message<T>(response: HttpResponse<T>) -> HttpResponse<T> {
    return response;
}

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
