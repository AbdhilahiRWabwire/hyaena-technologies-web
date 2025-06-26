#![allow(dead_code)]

use std::{primitive::str, vec::Vec};

// Hypertext Transfer Protocol Version Definition
pub type HTTPVersion = &'static str;

// Hypertext Transfer Protocol Versions
pub const HTTP_VERSION_ONE: HTTPVersion = "HTTP/1.1";
pub const HTTP_VERSION_TWO: HTTPVersion = "HTTP/2.0";
pub const HTTP_VERSION_THREE: HTTPVersion = "HTTP/3.0";

// Hypertext Transfer Protocol Version Vector
pub fn hypertext_transfer_versions() -> Vec<HTTPVersion> {
    let http_versions: Vec<HTTPVersion> =
        Vec::from([HTTP_VERSION_ONE, HTTP_VERSION_TWO, HTTP_VERSION_THREE]);

    return http_versions;
}
