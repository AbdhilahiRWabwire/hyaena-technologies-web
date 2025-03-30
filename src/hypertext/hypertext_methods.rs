#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{
    string::String,
    vec::Vec
};

// Hypertext Transfer Protocol Methods
pub fn http_methods() -> Vec<String> {
    let hypertext_methods: Vec<String> = Vec::from([
        "CONNECT",
        "DELETE",
        "GET",
        "HEAD",
        "OPTIONS",
        "PATCH",
        "POST",
        "PUT",
        "TRACE"
    ]);

    return hypertext_methods;
}
