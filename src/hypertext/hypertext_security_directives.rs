#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{string::String, vec::Vec};

// Hypertext Transfer Protocol Content Security Policy Directives
pub fn http_security_directives() -> Vec<String> {
    let hypertext_security_directives: Vec<String> = Vec::from([
        "base-uri".to_string(),
        "child-src".to_string(),
        "connect-src".to_string(),
        "default-src".to_string(),
        "font-src".to_string(),
        "form-action".to_string(),
        "frame-ancestors".to_string(),
        "frame-src".to_string(),
        "img-src".to_string(),
        "manifest-src".to_string(),
        "media-src".to_string(),
        "object-src".to_string(),
        "report-to".to_string(),
        "require-trusted-types-for".to_string(),
        "sandbox".to_string(),
        "script-src".to_string(),
        "script-src-attr".to_string(),
        "script-src-elem".to_string(),
        "style-src".to_string(),
        "style-src-attr".to_string(),
        "style-src-elem".to_string(),
        "trusted-types".to_string(),
        "upgrade-insecure-requests".to_string(),
        "worker-src".to_string(),
    ]);

    return hypertext_security_directives;
}
