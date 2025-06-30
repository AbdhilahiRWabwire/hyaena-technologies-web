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

use crate::hypertext_transfer::{
    http_headers::HTTP_CONTENT_LENGTH,
    http_status_codes::{HTTP_OK, HTTP_TWO_HUNDRED},
    http_versions::HTTP_VERSION_ONE,
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
pub fn request_message<T>(request: HttpRequest<T>) -> () {
    return ();
}

// Hypertext Transfer Protocol Response Message
pub fn response_message<T>(response: HttpResponse<T>) -> () {
    return ();
}

// Hypertext Transfer Protocol Connection Management
pub fn manage_connection(transmission_stream: &mut TcpStream) -> () {
    let mut standard_output: StdoutLock = stdout().lock();
    let mut buffered_reader: BufReader<&TcpStream> = BufReader::new(&transmission_stream);
    let mut stream_buffer: String = String::new();
    let source_path: PathBuf = PathBuf::from("./web/source/main.js");
    let source_file: Result<File, Error> = File::open(source_path);
    let mut file_buffer: String = String::new();
    let content_length: usize = file_buffer.len();

    buffered_reader.read_to_string(&mut stream_buffer).unwrap();
    writeln!(standard_output, "Hypertext Tranfer Protocol Request: ").unwrap();
    writeln!(standard_output, "").unwrap();
    writeln!(standard_output, "{}", stream_buffer).unwrap();

    match source_file {
        Ok(mut file) => {
            file.read_to_string(&mut file_buffer).unwrap();
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
            writeln!(transmission_stream, "{}", file_buffer).unwrap();
        }
        Err(error) => {
            eprintln!("Error Failed to Open Source File: {}", error);
            exit(1);
        }
    };

    return ();
}
