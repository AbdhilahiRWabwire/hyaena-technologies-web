#![allow(dead_code)]

use std::{net::TcpStream, thread, thread::JoinHandle, vec::Vec};

use super::{
    http_headers::HttpHeader,
    http_message::HttpBody,
    http_methods::HttpMethod,
    http_security_directives::HttpSecurityDirective,
    http_status_codes::{HttpStatusCode, HttpStatusText},
    http_versions::HttpVersion,
};

// Hypertext Transfer Protocol Response Definition
pub struct HttpResponse<T> {
    pub body: Option<HttpBody<T>>,
    pub headers: Vec<HttpHeader>,
    pub method: HttpMethod,
    pub security_directives: Option<Vec<HttpSecurityDirective>>,
    pub status_code: HttpStatusCode,
    pub status_text: HttpStatusText,
    pub transmission_stream: TcpStream,
    pub version: HttpVersion,
}

// Hypertext Transfer Protocol Response Message
pub fn response_message<T>(response: HttpResponse<T>) -> HttpResponse<T> {
    let standard_thread: JoinHandle<()> = thread::spawn(move || {});

    standard_thread.join().unwrap();

    return response;
}
