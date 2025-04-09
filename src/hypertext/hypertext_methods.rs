#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{string::String, vec::Vec};

// Hypertext Transfer Protocol Methods
pub fn http_methods() -> Vec<String> {
    let hypertext_methods: Vec<String> = Vec::from([
        "CONNECT".to_string(),
        "DELETE".to_string(),
        "GET".to_string(),
        "HEAD".to_string(),
        "OPTIONS".to_string(),
        "PATCH".to_string(),
        "POST".to_string(),
        "PUT".to_string(),
        "TRACE".to_string(),
    ]);

    return hypertext_methods;
}
