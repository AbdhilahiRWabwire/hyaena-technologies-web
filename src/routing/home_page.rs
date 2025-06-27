use std::{
    fs::File,
    io::{Error, Read, Write},
    net::TcpStream,
    path::PathBuf,
    primitive::usize,
    result::{
        Result,
        Result::{Err, Ok},
    },
    string::String,
};

use crate::hypertext_transfer::{
    http_headers::HTTP_CONTENT_LENGTH,
    http_status_codes::{HTTP_OK, HTTP_TWO_HUNDRED},
    http_versions::HTTP_VERSION_ONE,
};

// Application Home Page Route
pub fn home_route(transmission_stream: &mut TcpStream) -> () {
    let source_path: PathBuf = PathBuf::from("./web/source/main.js");
    let source_file: Result<File, Error> = File::open(source_path);
    let mut file_buffer: String = String::new();
    let content_length: usize = file_buffer.len();

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
            eprintln!("Error Failed to Open File: {}", error);
        }
    };

    return ();
}
