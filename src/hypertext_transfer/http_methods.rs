#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Hypertext Transfer Protocol Method Defintion
pub type HTTPMethod = &'static str;

// Hypertext Transfer Protocol Methods
pub const HTTP_CONNECT: HTTPMethod = "CONNECT";
pub const HTTP_DELETE: HTTPMethod = "DELETE";
pub const HTTP_GET: HTTPMethod = "GET";
pub const HTTP_HEAD: HTTPMethod = "HEAD";
pub const HTTP_OPTIONS: HTTPMethod = "OPTIONS";
pub const HTTP_PATCH: HTTPMethod = "PATCH";
pub const HTTP_POST: HTTPMethod = "POST";
pub const HTTP_PUT: HTTPMethod = "PUT";
pub const HTTP_TRACE: HTTPMethod = "TRACE";

// Hypertext Transfer Protocol Method Vector
pub fn methods_vector() -> Vec<HTTPMethod> {
    let http_methods: Vec<HTTPMethod> = Vec::from([
        HTTP_CONNECT,
        HTTP_DELETE,
        HTTP_GET,
        HTTP_HEAD,
        HTTP_OPTIONS,
        HTTP_PATCH,
        HTTP_POST,
        HTTP_PUT,
        HTTP_TRACE,
    ]);

    return http_methods;
}
