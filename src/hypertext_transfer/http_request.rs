#![allow(dead_code)]

use std::{option::Option, string::String, thread, thread::JoinHandle, vec::Vec};

use crate::hypertext_transfer::http_connection::{HttpBody, HttpConnection};

use super::{
    http_headers::HttpHeader,
    http_methods::HttpMethod,
    http_security_directives::HttpSecurityDirective,
    http_status_codes::{HttpStatusCode, HttpStatusText},
    http_versions::HttpVersion,
};

// Hypertext Transfer Protocol Request Definition
pub struct HttpRequest<T> {
    pub connection: HttpConnection,
    pub body: Option<HttpBody<T>>,
    pub headers: Vec<HttpHeader>,
    pub method: HttpMethod,
    pub security_directives: Option<Vec<HttpSecurityDirective>>,
    pub status_code: HttpStatusCode,
    pub status_text: HttpStatusText,
    pub url: String,
    pub version: HttpVersion,
}

// Hypertext Transfer Protocol Request Message
pub fn request_message<T>(request: HttpRequest<T>) -> HttpRequest<T> {
    let standard_thread: JoinHandle<()> = thread::spawn(move || {});

    standard_thread.join().unwrap();

    return request;
}
