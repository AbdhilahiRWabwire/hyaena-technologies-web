#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{
    string::String,
    vec::Vec
};

// Hypertext Transfer Protocol Content Security Policy Directives
pub fn http_security_directives() -> Vec<String> {
    let hypertext_security_directives: Vec<String> = Vec::from([
        "base-uri",
        "child-src",
        "connect-src",
        "default-src",
        "font-src",
        "form-action",
        "frame-ancestors",
        "frame-src",
        "img-src",
        "manifest-src",
        "media-src",
        "object-src",
        "report-to",
        "require-trusted-types-for",
        "sandbox",
        "script-src",
        "script-src-attr",
        "script-src-elem",
        "style-src",
        "style-src-attr",
        "style-src-elem",
        "trusted-types",
        "upgrade-insecure-requests",
        "worker-src"
    ]);

    return hypertext_security_directives;
}
