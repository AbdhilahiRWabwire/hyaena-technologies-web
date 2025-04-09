#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::{collections::HashMap, string::String};

// Hypertext Transfer Protocol Status Codes
pub fn http_status_codes() -> HashMap<String, String> {
    let hypertext_status_codes: HashMap<String, String> = HashMap::from([
        ("100".to_string(), "Continue".to_string()),
        ("101".to_string(), "Switching Protocols".to_string()),
        ("102".to_string(), "Processing".to_string()),
        ("103".to_string(), "Early Hints".to_string()),
        ("200".to_string(), "OK".to_string()),
        ("201".to_string(), "Created".to_string()),
        ("202".to_string(), "Accepted".to_string()),
        (
            "203".to_string(),
            "Non-Authoritative Information".to_string(),
        ),
        ("204".to_string(), "No Content".to_string()),
        ("205".to_string(), "Reset Content".to_string()),
        ("206".to_string(), "Partial Content".to_string()),
        ("207".to_string(), "Multi-Status".to_string()),
        ("208".to_string(), "Already Reported".to_string()),
        ("226".to_string(), "IM Used".to_string()),
        ("300".to_string(), "Multiple Choices".to_string()),
        ("301".to_string(), "Moved Permanently".to_string()),
        ("302".to_string(), "Found".to_string()),
        ("303".to_string(), "See Other".to_string()),
        ("304".to_string(), "Not Modified".to_string()),
        ("307".to_string(), "Temporary Redirect".to_string()),
        ("308".to_string(), "Permanent Redirect".to_string()),
        ("400".to_string(), "Bad Request".to_string()),
        ("401".to_string(), "Unauthorized".to_string()),
        ("402".to_string(), "Payment Required".to_string()),
        ("403".to_string(), "Forbidden".to_string()),
        ("404".to_string(), "Not Found".to_string()),
        ("405".to_string(), "Method Not Allowed".to_string()),
        ("406".to_string(), "Not Acceptable".to_string()),
        (
            "407".to_string(),
            "Proxy Authentication Required".to_string(),
        ),
        ("408".to_string(), "Request Timeout".to_string()),
        ("409".to_string(), "Conflict".to_string()),
        ("410".to_string(), "Gone".to_string()),
        ("411".to_string(), "Length Required".to_string()),
        ("412".to_string(), "Precondition Failed".to_string()),
        ("413".to_string(), "Content Too Large".to_string()),
        ("414".to_string(), "URI Too Long".to_string()),
        ("415".to_string(), "Unsupported Media Type".to_string()),
        ("416".to_string(), "Range Not Satisfiable".to_string()),
        ("417".to_string(), "Expectation Failed".to_string()),
        ("418".to_string(), "I'm a teapot".to_string()),
        ("421".to_string(), "Misdirected Request".to_string()),
        ("422".to_string(), "Unprocessable Content".to_string()),
        ("423".to_string(), "Locked".to_string()),
        ("424".to_string(), "Failed Dependency".to_string()),
        ("425".to_string(), "Too Early".to_string()),
        ("426".to_string(), "Upgrade Required".to_string()),
        ("428".to_string(), "Precondition Required".to_string()),
        ("429".to_string(), "Too Many Requests".to_string()),
        (
            "431".to_string(),
            "Request Header Fields Too Large".to_string(),
        ),
        (
            "451".to_string(),
            "Unavailable For Legal Reasons".to_string(),
        ),
        ("500".to_string(), "Internal Server Error".to_string()),
        ("501".to_string(), "Not Implemented".to_string()),
        ("502".to_string(), "Bad Gateway".to_string()),
        ("503".to_string(), "Service Unavailable".to_string()),
        ("504".to_string(), "Gateway Timeout".to_string()),
        ("505".to_string(), "HTTP Version Not Supported".to_string()),
        ("506".to_string(), "Variant Also Negotiates".to_string()),
        ("507".to_string(), "Insufficient Storage".to_string()),
        ("508".to_string(), "Loop Detected".to_string()),
        ("510".to_string(), "Not Extended".to_string()),
        (
            "511".to_string(),
            "Network Authentication Required".to_string(),
        ),
    ]);

    return hypertext_status_codes;
}
